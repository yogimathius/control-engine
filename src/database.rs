use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};
use std::env;

pub async fn connect_database() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/codex_sacred".to_string());

    // Create database if it doesn't exist
    if !Postgres::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        println!("🔮 Creating sacred database...");
        Postgres::create_database(&database_url).await?;
    }

    // Connect to the database
    let pool = PgPool::connect(&database_url).await?;

    println!("✨ Connected to sacred database");
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    println!("🏛️  Running sacred migrations...");
    sqlx::migrate!("./migrations").run(pool).await?;
    println!("✅ Sacred schema initialized");
    Ok(())
}
