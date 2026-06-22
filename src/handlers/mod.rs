pub mod region;

use std::net::SocketAddr;

use axum::{extract::ConnectInfo, http::StatusCode, Json};
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

/// 查看客户端远端 IP 地址
pub async fn client_ip(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Json<models::IpResponse> {
    Json(models::IpResponse {
        ip: addr.ip().to_string(),
        port: addr.port(),
    })
}
