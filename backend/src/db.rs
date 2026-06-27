use sqlx::{postgres::PgPoolOptions, PgPool};
use anyhow::{Context, Result};
use tracing::info;
use std::time::Duration;

pub type DbPool = PgPool;

/// Establishes an asynchronous connection pool with Postgres.
/// Leverages environment configuration, tunes connection management parameters
/// for concurrent production environments, and runs pending SQLx database migrations.
pub async fn init_pool(database_url: &str) -> Result<DbPool> {
    info!("Connecting to PostgreSQL database...");
    
    // Configure PgPoolOptions with optimal production values:
    // - max_connections: Upper limit of concurrent database connections
    // - min_connections: Keep a small set of connections warm to eliminate latency on initial requests
    // - acquire_timeout: Bound query wait times during extreme socket contention
    // - idle_timeout: Reap inactive sockets to free up database server resources
    // - max_lifetime: Cycle older connections to safeguard against socket leaks or dynamic router reclaims
    let pool = PgPoolOptions::new()
        .max_connections(25)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(120))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
        .context("Failed to build SQLx Postgres pool connection")?;

    info!("Database connection pool successfully initialized. Triggering automatic SQLx migrations...");
    
    // In cluster deployments, migrations run as a self-healing boot step inline with the service.
    // This prevents schema out-of-sync discrepancies across dynamic rollouts.
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Unable to run pending SQLx database migrations")?;

    info!("Database schemas successfully validated and updated.");
    
    Ok(pool)
}
