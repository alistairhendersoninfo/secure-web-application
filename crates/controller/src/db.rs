use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

pub type Db = sqlx::Pool<sqlx::Postgres>;

pub async fn connect(database_url: &str) -> Result<Db> {
    // Enforce TLS via connection string (`sslmode=require`) and server cert verification
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;
    Ok(pool)
}

pub async fn migrate(pool: &Db) -> Result<()> {
    // Embed migrations from workspace root migrations folder
    // Path is relative to this crate (crates/controller)
    sqlx::migrate!("../../migrations").run(pool).await?;
    Ok(())
}
