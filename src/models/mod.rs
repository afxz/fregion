use serde::Serialize;

/// Ping 接口响应
#[derive(Debug, Serialize)]
pub struct PingResponse {
    pub message: String,
}
