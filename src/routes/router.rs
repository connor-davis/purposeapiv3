use std::sync::Arc;

use axum::{
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use reqwest::StatusCode;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;

use super::authentication;
use crate::{config::Config, utils::jwt_authentication::auth, AppState};

pub async fn router() -> Router {
    let env_result = dotenv();

    match env_result {
        Ok(_) => {
            let config = Config::init();

            let pool = match PgPoolOptions::new()
                .max_connections(10)
                .connect(&config.database_url)
                .await
            {
                Ok(pool) => {
                    println!("✅ Connection to the database is successful!");
                    pool
                }
                Err(err) => {
                    println!("🔥 Failed to connect to the database: {:?}", err);
                    std::process::exit(1);
                }
            };

            let migration_result = sqlx::migrate!().run(&pool).await;

            let app_state = Arc::new(AppState {
                db: pool.clone(),
                env: config.clone(),
            });

            match migration_result {
                Ok(_) => Router::new()
                    .route("/", get(index))
                    .nest(
                        "/authentication",
                        Router::new()
                            .route("/register", post(authentication::register::index))
                            .route("/login", post(authentication::login::index))
                            .nest(
                                "/",
                                Router::new()
                                    .route("/logout", get(authentication::logout::index))
                                    .route("/me", get(authentication::me::index))
                                    .route_layer(middleware::from_fn_with_state(
                                        app_state.clone(),
                                        auth,
                                    )),
                            ),
                    )
                    .fallback(fallback)
                    .with_state(app_state),
                Err(error) => {
                    println!(
                        "Failed to run database migrations: {}, Using limited router.",
                        error
                    );

                    Router::new()
                        .route("/", get(limited_index))
                        .fallback(fallback)
                }
            }
        }
        Err(error) => {
            println!(
                "Failed to run database migrations: {}, Using limited router.",
                error
            );

            Router::new()
                .route("/", get(limited_index))
                .fallback(fallback)
        }
    }
}

async fn index() -> impl IntoResponse {
    Json(json!({
        "status": StatusCode::OK.as_u16(),
        "message": "Welcome to PurposeAPI v3",
        "database": true
    }))
}

async fn limited_index() -> impl IntoResponse {
    Json(json!({
        "status": StatusCode::OK.as_u16(),
        "message": "Welcome to PurposeAPI v3",
        "database": false
    }))
}

async fn fallback() -> impl IntoResponse {
    Json(json!({
        "status": StatusCode::NOT_FOUND.as_u16(),
        "message": "Route not found. Please contact the developer.",
    }))
}
