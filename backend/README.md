# Agent Duel Backend Codebase (Base Ecosystem)

A high-performance, async, production-grade Rust backend running the **Agent Duel** crash routing game. Connects to a PostgreSQL database using SQLx, handles requests asynchronously with Axum, implements structured logging via tracing, and encapsulates errors using anyhow.

## Architecture and File Map

```text
/backend
├── Cargo.toml            # Package manifest with Axum, SQLx, tracing, and anyhow
├── Dockerfile            # Multi-stage container production recipe (cargo-chef optimized)
├── docker-compose.yml    # Main services stack configuration (App + DB)
├── migrations/           # Database migration files managed automatically by SQLx
│   └── *_init.sql        # Table structures for transactions, wallets, and leaderboard scores
├── tests/
│   └── integration_tests.rs # Automated system tests for API stability validation
├── src/
│   ├── main.rs           # Entry point: tracing setup, environment loading, server loop
│   ├── lib.rs            # Library module mapping (exposes submodules for tests)
│   ├── db.rs             # Database initializing pool and migrations runner
│   ├── error.rs          # Adaptable anyhow Error converter translating server issues into secure JSON responses
│   ├── models.rs         # Domain structures (Wallets, GameRounds, WagerRecords, Leaderboard)
│   ├── handlers.rs       # Controller logic (wallet register, wager commit, live leaderboard)
│   └── routes.rs         # Expressive Axum routing setups with secure CORS filters
```

---

## Technical Onboarding and Environment Setup

### 1. Prerequisites
Before beginning, ensure you have the following installed on your machine:
- **Rust Toolchain**: Stable edition (v1.78.0+) - Install via [rustup](https://rustup.rs/)
- **Docker & Docker Compose**: For containerized deployment or sandbox services
- **SQLx CLI** (Optional, for managing schema updates locally):
  ```bash
  cargo install sqlx-cli --no-default-features --features postgres
  ```

### 2. Sandbox Setup in 2-Clicks
To spin up the entire production-grade stack including the Rust API and the Postgres database, run:
```bash
docker-compose up --build
```
This performs a multi-stage compilation, launches PostgreSQL, verifies host database connection health, and triggers automatic schema validation/migration.

### 3. Local Non-Docker Development Execution
1. Create a local environment configuration `.env` file inside `/backend`:
   ```env
   DATABASE_URL=postgres://postgres:secure_db_password_x402@localhost:5432/agent_duel
   PORT=3000
   RUST_LOG=agent_duel_backend=info,tower_http=info,sqlx=warn
   ```
2. Spawn PostgreSQL locally, then trigger migrations:
   ```bash
   sqlx database setup
   ```
3. Run the application locally with file watching enabled:
   ```bash
   cargo run
   ```

### 4. Running Automated Integrations Tests
Execute our integration checks at any development milestone to guarantee stability:
```bash
cargo test
```

---

## Core Protocols of Veklom Base Mainnet Integrations

### Identity Verification
- All users log in using their **Veklom ID Wallet** (`0x3a74772e925b54F7dAD7FD95c9Ba30825033f970`), which is checked securely against the authentication service at `veklom-id.vercel.app`.

### Escrow Facilitator
- Round stakes and smart transaction rewards are managed asynchronously via the **Escrow Facilitator Registry** contract at (`0xCC34553b4e6332ffb9C1b61E22436ACA53113D1d`). Real-time database synchronizations track total capital secured with automated payouts triggered upon manual or automatic eject triggers.
