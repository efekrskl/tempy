mod http;

use anyhow::Context;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let db_connect_options = SqliteConnectOptions::new()
        .filename("db/db.sqlite")
        .create_if_missing(true);
    let db = SqlitePool::connect_with(db_connect_options)
        .await
        .context("Couldn't connect to the DB.")?;
    sqlx::migrate!("db/migrations").run(&db).await?;

    http::serve(db).await?;

    Ok(())
}
