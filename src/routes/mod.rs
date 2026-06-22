use axum::{routing::get, Router};
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    trace::TraceLayer,
};

use crate::handlers;

/// 创建并配置应用路由
pub fn create_router() -> Router {
    // 全局中间件
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let middleware = tower::ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(cors)
        .layer(CatchPanicLayer::new());

    // API 路由
    Router::new()
        .route("/health", get(handlers::health_check))
        .nest("/api/v1", api_routes())
        .layer(middleware)
}

/// `/api/v1` 子路由
fn api_routes() -> Router {
    Router::new().route("/ping", get(handlers::ping))
}
