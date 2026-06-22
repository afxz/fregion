use axum::{routing::get, Router};
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    trace::TraceLayer,
};

use crate::handlers::{self, region::AppState};

/// 创建并配置应用路由
pub fn create_router(state: AppState) -> Router {
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
        .with_state(state)
}

/// `/api/v1` 子路由
fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/ping", get(handlers::ping))
        .route("/ip", get(handlers::client_ip))
        .nest("/regions", region_routes())
}

/// 区域相关路由
fn region_routes() -> Router<AppState> {
    Router::new()
        .route("/countries", get(handlers::region::list_countries))
        .route(
            "/countries/{country_id}/provinces",
            get(handlers::region::list_provinces),
        )
        .route(
            "/provinces/{province_id}/cities",
            get(handlers::region::list_cities),
        )
        .route(
            "/cities/{city_id}/districts",
            get(handlers::region::list_districts),
        )
}
