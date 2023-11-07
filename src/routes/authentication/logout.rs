use axum::{http::Response, response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use reqwest::{header, StatusCode};
use serde_json::{json, Value};

pub async fn index() -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}
