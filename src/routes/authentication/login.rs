use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::{header, StatusCode};
use serde_json::{json, Value};

use crate::{
    models::{login_user::LoginUser, token_claim::TokenClaims, user::User},
    AppState,
};

pub async fn index(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT * FROM users WHERE email = $1;
        "#,
        body.email.to_ascii_lowercase()
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|error| {
        let error_response = json!({
            "status": "INTERNAL_SERVER_ERROR",
            "message": format!("Failed to find user in database: {}", error)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = json!({
            "status": "BAD_REQUEST",
            "message": "Invalid email or password."
        });

        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = json!({
            "status": "BAD_REQUEST",
            "message": "Invalid email or password."
        });

        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

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
            "token": token
        })
        .to_string(),
    );

    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}
