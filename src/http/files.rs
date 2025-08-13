use crate::http::ApiContext;
use axum::routing::post;
use axum::{Extension, Router};

pub fn router() -> Router {
    Router::new().route("/api/files", post(upload))
}

async fn upload(ctx: Extension<ApiContext>) -> &'static str {
    "Not implemented!"
}
