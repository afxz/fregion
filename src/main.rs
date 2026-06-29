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
    // Windows 下通过释放控制台来模拟守护进程行为
    unsafe { windows_detach_console() };
    tracing::info!("已释放控制台，进入后台运行模式");
}

#[cfg(windows)]
unsafe fn windows_detach_console() {
    extern "system" {
        fn FreeConsole() -> i32;
    }
    FreeConsole();
}
