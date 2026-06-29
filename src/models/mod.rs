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
    /// 是否通过代理头（X-Forwarded-For / X-Real-IP）获取
    #[serde(skip_serializing_if = "is_false")]
    pub forwarded: bool,
}

fn is_false(v: &bool) -> bool {
    !*v
}
