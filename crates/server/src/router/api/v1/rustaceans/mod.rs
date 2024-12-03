mod retrieve;

use axum::routing::get;
use axum::Router;

pub fn routes() -> Router {
    Router::new().route("/", get(retrieve::handler))
}
