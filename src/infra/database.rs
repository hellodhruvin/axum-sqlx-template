use sqlx::pool::Pool;
use sqlx::postgres::{PgPoolOptions, Postgres};

pub type DbPool = Pool<Postgres>;

pub async fn create_connection(db_url: String) -> anyhow::Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(db: DbPool) -> anyhow::Result<()> {
    sqlx::migrate!().run(&db).await?;

    Ok(())
}
