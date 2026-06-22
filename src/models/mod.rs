pub mod region;

use serde::Serialize;

/// Ping 接口响应
#[derive(Debug, Serialize)]
pub struct PingResponse {
    pub message: String,
}

/// 客户端 IP 接口响应
#[derive(Debug, Serialize)]
pub struct IpResponse {
    pub ip: String,
    pub port: u16,
}
