mod api;

use axum::Router;
use http::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn make_router() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new().nest("/api/v1", api::v1::routes()).layer(cors)
}
