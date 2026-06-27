use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletRegistry {
    pub address: String,
    pub id_wallet: String,
    pub payment_wallet: String,
    pub verification_domain: String,
    pub network: String,
    pub verified_at: NaiveDateTime,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameRound {
    pub round_id: Uuid,
    pub game_index: i64,
    pub crash_multiplier: f64,
    pub winning_agent: String, // 'A' (Vector North) or 'B' (Quiet Switch)
    pub started_at: NaiveDateTime,
    pub ended_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WagerRecord {
    pub wager_id: Uuid,
    pub round_id: Uuid,
    pub wallet_address: String,
    pub selected_agent: String,
    pub wager_amount_usdc: f64,
    pub cashout_multiplier: Option<f64>,
    pub payout_amount_usdc: f64,
    pub tx_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaderboardEntry {
    pub rank: i32,
    pub wallet_address: String,
    pub total_won_usdc: f64,
    pub best_multiplier: f64,
    pub streak: i32,
    pub total_rounds_played: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FacilitatorConfig {
    pub key_id: String,
    pub contract_address: String,
    pub total_secured_usdc: f64,
    pub fee_basis_points: i32, // e.g. 150 BPS = 1.5%
    pub active_facilitators: Vec<String>,
}
