use anyhow::Context;
use axum::routing::get;
use axum::{Extension, Router};
use sqlx::SqlitePool;
use tower::ServiceBuilder;

#[derive(Clone, Debug)]
struct ApiContext {
    db: SqlitePool,
}

pub async fn serve(db: SqlitePool) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(say_hello))
        .layer(ServiceBuilder::new().layer(Extension(ApiContext { db })));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app)
        .await
        .context("Couldn't start the http server")
}

async fn say_hello(ctx: Extension<ApiContext>) -> &'static str {
    "Hello World!"
}
