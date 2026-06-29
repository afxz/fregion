pub mod region;

use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::ConnectInfo,
    http::{HeaderMap, StatusCode},
    Json,
};
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

/// 查看客户端远端 IP 地址，支持代理头穿透
pub async fn client_ip(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Json<models::IpResponse> {
    // 尝试从 X-Forwarded-For 获取真实 IP（取最左侧第一个）
    if let Some(ip) = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse::<IpAddr>().ok())
    {
        return Json(models::IpResponse {
            ip: ip.to_string(),
            port: 0,
            forwarded: true,
        });
    }

    // 回退到 X-Real-IP
    if let Some(ip) = headers
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<IpAddr>().ok())
    {
        return Json(models::IpResponse {
            ip: ip.to_string(),
            port: 0,
            forwarded: true,
        });
    }

    // 最终回退到直连 IP
    Json(models::IpResponse {
        ip: addr.ip().to_string(),
        port: addr.port(),
        forwarded: false,
    })
}
