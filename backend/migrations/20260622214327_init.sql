-- Init SQL schema for Agent Duel game database
-- Structured matching Base Mainnet cross-platform dApp transactions

CREATE TABLE IF NOT EXISTS wallet_registry (
    address VARCHAR(42) PRIMARY KEY,
    id_wallet VARCHAR(42) NOT NULL,
    payment_wallet VARCHAR(42) NOT NULL,
    verification_domain VARCHAR(100) NOT NULL,
    network VARCHAR(50) NOT NULL,
    verified_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL
);

CREATE TABLE IF NOT EXISTS game_rounds (
    round_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    game_index BIGSERIAL NOT NULL,
    crash_multiplier DOUBLE PRECISION NOT NULL,
    winning_agent VARCHAR(1) NOT NULL, -- 'A' (Vector North) or 'B' (Quiet Switch)
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ended_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS wager_records (
    wager_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    round_id UUID NOT NULL,
    wallet_address VARCHAR(42) NOT NULL,
    selected_agent VARCHAR(1) NOT NULL, -- 'A' or 'B'
    wager_amount_usdc DOUBLE PRECISION NOT NULL,
    cashout_multiplier DOUBLE PRECISION,
    payout_amount_usdc DOUBLE PRECISION NOT NULL,
    tx_hash VARCHAR(66) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_wager_records_wallet ON wager_records(wallet_address);
CREATE INDEX IF NOT EXISTS idx_wager_records_round ON wager_records(round_id);

CREATE TABLE IF NOT EXISTS leaderboard (
    wallet_address VARCHAR(42) PRIMARY KEY,
    total_won_usdc DOUBLE PRECISION DEFAULT 0.0 NOT NULL,
    best_multiplier DOUBLE PRECISION DEFAULT 1.0 NOT NULL,
    streak INT DEFAULT 0 NOT NULL,
    total_rounds_played INT DEFAULT 0 NOT NULL
);

CREATE TABLE IF NOT EXISTS facilitator_config (
    key_id VARCHAR(50) PRIMARY KEY,
    contract_address VARCHAR(42) NOT NULL,
    total_secured_usdc DOUBLE PRECISION DEFAULT 0.0 NOT NULL,
    fee_basis_points INT DEFAULT 150 NOT NULL -- Default 1.5% fee
);

-- Seed Initial dynamic settings for easy onboarding and verification
INSERT INTO facilitator_config (key_id, contract_address, total_secured_usdc, fee_basis_points)
VALUES ('veklom_base_mainnet', '0xCC34553b4e6332ffb9C1b61E22436ACA53113D1d', 145890.30, 150)
ON CONFLICT (key_id) DO NOTHING;
