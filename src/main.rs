use std::net::SocketAddr;

use axum::{extract::DefaultBodyLimit, http::HeaderValue};
use config::Config;
use dotenv::dotenv;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use sqlx::{Pool, Postgres};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use crate::routes::router::router;

pub mod config;
pub mod models;
pub mod routes;
pub mod utils;

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[tokio::main]
async fn main() {
    let env_result = dotenv();

    match env_result {
        Ok(_) => {
            println!(
                r#"
000       0000 0000 0000      0000       000000               00000000      00000000    000       0000   00000000   0000      0000 0000    000000000000 0000 000       0000    00000000
00000     0000 0000 000000  00000       00000000             0000000000    0000000000   00000     0000  0000000000  0000      0000 0000    000000000000 0000 00000     0000   0000000000
000000    0000 0000   0000  0000       0000  0000           0000    0000  0000    0000  000000    0000 0000    0000 0000      0000 0000         00      0000 000000    0000  0000    0000
00000000  0000 0000    00000000       0000    0000         0000          0000      0000 00000000  0000  000000      0000      0000 0000         00      0000 00000000  0000 0000     
000000000 0000 0000      0000        0000      0000        0000          0000      0000 000000000 0000    000000    9999      0000 0000         00      0000 000000000 0000 0000  0000000
0000  00000000 0000      0000       0000000000000000       0000          0000      0000 0000  00000000       000000 0000      0000 0000         00      0000 0000  00000000 0000  0000000
0000   0000000 0000      0000      000000000000000000       0000    0000  0000    0000  0000   0000000 0000    0000 0000      0000 0000         00      0000 0000   0000000  0000    0000
0000    000000 0000      0000     0000            0000       0000000000    0000000000   0000    000000  0000000000    0000000000   000000000000 00      0000 0000    000000   0000000000
0000      0000 0000      0000    0000              0000       00000000      00000000    0000      0000   00000000        0000      000000000000 00      0000 0000      0000    00000000

"#
            );

            tracing_subscriber::registry()
                .with(tracing_subscriber::EnvFilter::new(
                    std::env::var("RUST_LOG")
                        .unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
                ))
                .with(tracing_subscriber::fmt::layer())
                .init();

            let app = router().await;

            let cors = CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_credentials(true)
                .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

            let app = app
                .layer(DefaultBodyLimit::max(100_000_000))
                .layer(cors)
                .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

            let address = SocketAddr::from(([0, 0, 0, 0], 3000));

            println!("Server listening on {}", address);

            axum::Server::bind(&address)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        Err(_) => {
            println!("Failed to find environment variables. Application will exit.")
        }
    }
}
