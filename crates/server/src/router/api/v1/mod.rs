mod rustaceans;

use axum::Router;

pub fn routes() -> Router {
    Router::new().nest("/rustaceans", rustaceans::routes())
}
