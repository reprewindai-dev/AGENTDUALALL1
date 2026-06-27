use std::net::SocketAddr;
use tracing::{info, warn};
use tracing_subscriber::{prelude::*, EnvFilter};
use dotenvy::dotenv;

mod db;
mod error;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize environment variables
    if let Err(_) = dotenv() {
        warn!(".env file omitted or inaccessible. Defaulting to system environment settings");
    }

    // 2. Initialize structured production logging (Json tracing subscriber) for excellent visibility
    let logs_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("agent_duel_backend=info,tower_http=info,sqlx=warn"));

    tracing_subscriber::registry()
        .with(logs_filter)
        .with(tracing_subscriber::fmt::layer().json()) // structured json for logging monitors like Datadog/Splunk
        .init();

    info!("Structured logging engine loaded. Powering up Agent Duel Core VM!");

    // 3. Resolve PostgreSQL instance URL
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/agent_duel".to_string());

    // 4. Initialize Database connection pool & run SQlx automatic migrations
    let pool = db::init_pool(&database_url).await?;

    // 5. Build and configure Axum routing map
    let app = routes::compile_routes(pool);

    // 6. Bind listener address
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!(address = %addr, "Axum server successfully bound. Ready for high stakes dApp pipelines");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
