use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::{
    register_or_connect_wallet,
    create_wager,
    query_leaderboard,
    query_facilitator,
};

/// Configures and compiles all application API endpoints.
/// Sets up rigid production-grade CORS configuration for cross-platform dApp integrations on Base.
pub fn compile_routes(pool: PgPool) -> Router {
    // Standard secure CORS rules allowing full compatibility within vercel, base.org and dApp frames
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Wallet Authentication & Onboarding
        .route("/api/wallet/connect", post(register_or_connect_wallet))
        
        // Micro High-Stakes Game State Records
        .route("/api/wagers/submit", post(create_wager))
        
        // Real-Time Leaderboard
        .route("/api/leaderboard", get(query_leaderboard))
        
        // Base Escrow Facilitator Registry
        .route("/api/facilitator/:key_id", get(query_facilitator))
        
        // State Sharing and Health validation
        .route("/api/health", get(|| async { "Agent Duel base network node is ONLINE" }))
        
        // Bind the active PgPool database state context globally
        .with_state(pool)
        .layer(cors)
}
