mod cli;
mod handlers;
mod models;
mod routes;

use std::net::SocketAddr;

use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::cli::Cli;
use crate::handlers::region::AppState;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "fregion=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 守护进程模式
    if cli.daemon {
        daemonize();
    }

    // 应用状态（传入数据目录）
    let state = AppState::new(&cli.data);

    // 构建路由
    let app = routes::create_router(state);

    // 启动服务
    let addr = SocketAddr::new(cli.addr, cli.port);
    tracing::info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

#[cfg(unix)]
fn daemonize() {
    use daemonize::Daemonize;

    Daemonize::new()
        .stdout(std::fs::File::create("fregion.out.log").unwrap())
        .stderr(std::fs::File::create("fregion.err.log").unwrap())
        .start()
        .unwrap_or_else(|e| {
            tracing::error!("守护进程启动失败: {}", e);
            std::process::exit(1);
        });
}

#[cfg(windows)]
fn daemonize() {
    use std::os::windows::process::CommandExt;
    use std::process::Command;

    let exe = std::env::current_exe().expect("无法获取当前可执行文件路径");

    // 传递给子进程的参数，去掉 --daemon / -d
    let args: Vec<String> = std::env::args()
        .skip(1)
        .filter(|a| a != "--daemon" && a != "-d")
        .collect();

    let stdout = std::fs::File::create("fregion.out.log").expect("无法创建 stdout 日志文件");
    let stderr = std::fs::File::create("fregion.err.log").expect("无法创建 stderr 日志文件");

    let child = Command::new(exe)
        .args(&args)
        .stdout(stdout)
        .stderr(stderr)
        .creation_flags(0x08000000 | 0x00000008) // CREATE_NO_WINDOW | DETACHED_PROCESS
        .spawn()
        .expect("无法启动守护进程");

    tracing::info!("守护进程已启动 (PID: {})", child.id());
    std::process::exit(0);
}
