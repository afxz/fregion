use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

use crate::models;

/// 健康检查
pub async fn health_check() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "service": "fregion",
        })),
    )
}

/// Ping 接口
pub async fn ping() -> Json<models::PingResponse> {
    Json(models::PingResponse {
        message: "pong".to_string(),
    })
}
