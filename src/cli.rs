use std::net::IpAddr;

use clap::Parser;

/// 区域数据 API 服务
#[derive(Parser, Debug)]
#[command(name = "fregion", version, about)]
pub struct Cli {
    /// 监听端口
    #[arg(short = 'p', long, default_value_t = 3000)]
    pub port: u16,

    /// 监听 IP 地址
    #[arg(short = 'a', long, default_value = "127.0.0.1")]
    pub addr: IpAddr,

    /// 数据目录路径
    #[arg(short = 'D', long, default_value = "data")]
    pub data: String,

    /// 以守护进程模式启动
    #[arg(short = 'd', long, default_value_t = false)]
    pub daemon: bool,
}
