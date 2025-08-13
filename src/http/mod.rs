mod files;

use anyhow::Context;
use axum::Extension;
use sqlx::SqlitePool;
use tower::ServiceBuilder;

#[derive(Clone, Debug)]
pub struct ApiContext {
    db: SqlitePool,
}

pub async fn serve(db: SqlitePool) -> anyhow::Result<()> {
    let router = files::router();
    let app = router.layer(ServiceBuilder::new().layer(Extension(ApiContext { db })));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app)
        .await
        .context("Couldn't start the http server")?;

    println!("Listening on port :3000");

    Ok(())
}
