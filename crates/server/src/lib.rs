mod router;

use axum::body::Body;
use axum::http::Response;
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest, Result};

use self::router::make_router;

#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response<Body>> {
    console_error_panic_hook::set_once();
    Ok(make_router().call(req).await?)
}
