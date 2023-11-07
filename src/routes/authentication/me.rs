use axum::{response::IntoResponse, Extension, Json};
use reqwest::StatusCode;

use crate::{models::user::User, utils::filters::filter_user};

pub async fn index(
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let filtered_user = filter_user(&user).await;

    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": filtered_user
        })
    });

    Ok(Json(json_response))
}
