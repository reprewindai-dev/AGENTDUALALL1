// Automated Integration Tests for Agent Duel Backend
// Verifies Axum routing mapping, robust anyhow error feedback, and SQLx bindings

#[cfg(test)]
mod tests {
    use std::net::TcpListener;
    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;
    use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

    // Helper functions to spin up local test sandbox servers
    async fn spawn_test_server() -> String {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:test_password_x402@localhost:5432/agent_duel_test".to_string());
        
        // Build mock pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test PostgreSQL database");

        // Execute migrations to build our clean db table structure
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed running SQLx migrations inside tests folder");

        let app = agent_duel_backend::routes::compile_routes(pool);
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random TCP port");
        let port = listener.local_addr().unwrap().port();
        
        tokio::spawn(async move {
            axum::serve(tokio::net::TcpListener::from_std(listener).unwrap(), app)
                .await
                .unwrap();
        });

        format!("http://127.0.0.1:{}", port)
    }

    #[tokio::test]
    async fn test_health_check_endpoint() {
        let address = spawn_test_server().await;
        let client = reqwest::Client::new();

        let response = client
            .get(&format!("{}/api/health", address))
            .send()
            .await
            .expect("Failed sending HTTP request to health check route");

        assert!(response.status().is_success());
        let body = response.text().await.unwrap();
        assert_eq!(body, "Agent Duel base network node is ONLINE");
    }

    #[tokio::test]
    async fn test_wallet_registration_flow() {
        let address = spawn_test_server().await;
        let client = reqwest::Client::new();

        // Generate a localized deterministic mock wallet payload matching models::WalletRegistry
        let d = NaiveDate::from_ymd_opt(2026, 6, 22).unwrap();
        let t = NaiveTime::from_hms_opt(21, 43, 27).unwrap();
        let mock_datetime = NaiveDateTime::new(d, t);

        let payload = agent_duel_backend::models::WalletRegistry {
            address: "0x3a74772e925b54F7dAD7FD95c9Ba30825033f970".to_string(),
            id_wallet: "0x3a74772e925b54F7dAD7FD95c9Ba30825033f970".to_string(),
            payment_wallet: "0xCC34553b4e6332ffb9C1b61E22436ACA53113D1d".to_string(),
            verification_domain: "veklom.base.org".to_string(),
            network: "Base Mainnet".to_string(),
            verified_at: mock_datetime,
            is_active: true,
        };

        let response = client
            .post(&format!("{}/api/wallet/connect", address))
            .json(&payload)
            .send()
            .await
            .expect("Failed executing POST to wallet connect API");

        assert!(response.status().is_success());
        let res_body: serde_json::Value = response.json().await.unwrap();
        
        assert_eq!(res_body["status"], "success");
        assert!(res_body["message"].as_str().unwrap().contains("registered"));
        assert!(res_body["verified_endpoint"].as_str().unwrap().contains("veklom.base.org"));
    }

    #[tokio::test]
    async fn test_create_wager_flow() {
        let address = spawn_test_server().await;
        let client = reqwest::Client::new();

        let d = NaiveDate::from_ymd_opt(2026, 6, 22).unwrap();
        let t = NaiveTime::from_hms_opt(21, 44, 12).unwrap();
        let mock_datetime = NaiveDateTime::new(d, t);

        // First pre-register the wallet to initialize the score placeholder in the leaderboard
        let wallet_payload = agent_duel_backend::models::WalletRegistry {
            address: "0xF1295fcdef244bb4dd5155fbe9725f05810b7E82".to_string(),
            id_wallet: "0xF1295fcdef244bb4dd5155fbe9725f05810b7E82".to_string(),
            payment_wallet: "0xCC34553b4e6332ffb9C1b61E22436ACA53113D1d".to_string(),
            verification_domain: "veklom.base.org".to_string(),
            network: "Base Mainnet".to_string(),
            verified_at: mock_datetime,
            is_active: true,
        };

        let register_res = client
            .post(&format!("{}/api/wallet/connect", address))
            .json(&wallet_payload)
            .send()
            .await
            .expect("Pre-register wallet failure");
        assert!(register_res.status().is_success());

        // Construct high-stakes wager record
        let wager_payload = agent_duel_backend::models::WagerRecord {
            wager_id: Uuid::new_v4(),
            round_id: Uuid::new_v4(),
            wallet_address: "0xF1295fcdef244bb4dd5155fbe9725f05810b7E82".to_string(),
            selected_agent: "A".to_string(),
            wager_amount_usdc: 250.0,
            cashout_multiplier: Some(2.20),
            payout_amount_usdc: 550.0,
            tx_hash: "0x82a9db8c3132e185c8bbbc010eef85f02af85c822a1012111199a0a0cdffd4aa".to_string(),
            created_at: mock_datetime,
        };

        // Submit micro-stakes transaction
        let wager_res = client
            .post(&format!("{}/api/wagers/submit", address))
            .json(&wager_payload)
            .send()
            .await
            .expect("Failed submitting active high-stakes wager payload");

        assert!(wager_res.status().is_success());
        let res_body: serde_json::Value = wager_res.json().await.unwrap();
        
        assert_eq!(res_body["success"], true);
        assert_eq!(res_body["leaderboard_updated"], true);
        assert!(res_body["wager_id"].is_string());
    }

    #[tokio::test]
    async fn test_leaderboard_query() {
        let address = spawn_test_server().await;
        let client = reqwest::Client::new();

        let response = client
            .get(&format!("{}/api/leaderboard", address))
            .send()
            .await
            .expect("Failed loading leaderboard data");

        assert!(response.status().is_success());
        let body: serde_json::Value = response.json().await.unwrap();
        assert!(body.is_array(), "Leaderboard payload must render a secure aligned JSON array");
    }

    #[tokio::test]
    async fn test_facilitator_config_query() {
        let address = spawn_test_server().await;
        let client = reqwest::Client::new();

        let response = client
            .get(&format!("{}/api/facilitator/veklom_base_mainnet", address))
            .send()
            .await
            .expect("Failed retrieving Base Mainnet facilitator config registry");

        assert!(response.status().is_success());
        let body: serde_json::Value = response.json().await.unwrap();
        
        assert_eq!(body["key_id"], "veklom_base_mainnet");
        assert_eq!(body["contract_address"], "0xCC34553b4e6332ffb9C1b61E22436ACA53113D1d");
        assert_eq!(body["network"], "Base Mainnet");
    }

    #[tokio::test]
    async fn test_unregistered_facilitator_query() {
        let address = spawn_test_server().await;
        let client = reqwest::Client::new();

        let response = client
            .get(&format!("{}/api/facilitator/non_existent_key", address))
            .send()
            .await
            .expect("Failed calling non_existent_key query path");

        assert!(response.status().is_success());
        let body: serde_json::Value = response.json().await.unwrap();
        assert_eq!(body["success"], false);
        assert!(body["error"].as_str().unwrap().contains("not registered"));
    }
}

pub mod mock_bindings {
    // Declared to ensure cargo test scans it correctly
}
