use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{extract::State, http::Response, response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use reqwest::{header, StatusCode};
use serde_json::{json, Value};

use crate::{
    models::{register_user::RegisterUser, token_claim::TokenClaims, user::User},
    utils::filters::filter_user,
    AppState,
};

pub async fn index(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user_found: Option<bool> = sqlx::query_scalar(
        r#"
        SELECT EXISTS(SELECT 1 FROM users WHERE email = $1);
    "#,
    )
    .bind(body.email.to_owned().to_ascii_lowercase())
    .fetch_one(&state.db)
    .await
    .map_err(|error| {
        let error_response = json!({
            "status": "INTERNAL_SERVER_ERROR",
            "message": format!("Failed to find user in database: {}", error)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if let Some(found) = user_found {
        if found {
            let error_response = json!({
                "status": "CONFLICT",
                "message": "Email address is already in use."
            });

            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|error| {
            let error_response = json!({
                "status": "INTERNAL_SERVER_ERROR",
                "message": format!("Failed to hash users password: {}", error)
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password, user_group) VALUES ($1, $2, $3) RETURNING *;
    "#,
        body.email.to_ascii_lowercase(),
        password_hash,
        body.user_group
    )
    .fetch_one(&state.db)
    .await
    .map_err(|error| {
        let error_response = json!({
            "status": "INTERNAL_SERVER_ERROR",
            "message": format!("Failed to insert new user into the database: {}", error)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let filtered_user = filter_user(&user).await;

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        exp,
        iat,
        sub: user.id.to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(
        json!({
            "status": "SUCCESS",
            "token": token,
            "data": json!({
                "user": filtered_user
            })
        })
        .to_string(),
    );

    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}
