// Common test utilities
use std::env;
use mongodb::{Client, Database};

pub async fn setup_test_db() -> Database {
    let uri = env::var("MONGODB_TEST_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    
    let client = Client::with_uri_str(&uri)
        .await
        .expect("Failed to connect to test MongoDB");
    
    // Use timestamp-based unique DB name instead of uuid
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let db_name = format!("test_crypto_tracker_{}", timestamp);
    client.database(&db_name)
}

pub async fn cleanup_test_db(db: &Database) {
    let _ = db.drop(None).await;
}

pub fn init_test_logger() {
    let _ = env_logger::builder()
        .is_test(true)
        .try_init();
}

// Mock data generators
pub mod mock_data {
    use chrono::Utc;
    use serde::{Deserialize, Serialize};
    
    // Re-export models from main crate
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct CryptoToken {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<mongodb::bson::oid::ObjectId>,
        pub token_id: String,
        pub symbol: String,
        pub name: String,
        pub current_price: f64,
        pub market_cap: f64,
        pub volume_24h: f64,
        pub price_change_24h: f64,
        pub price_change_percentage_24h: f64,
        pub high_24h: Option<f64>,
        pub low_24h: Option<f64>,
        pub circulating_supply: Option<f64>,
        pub total_supply: Option<f64>,
        pub ath: Option<f64>,
        pub ath_change_percentage: Option<f64>,
        pub atl: Option<f64>,
        pub atl_change_percentage: Option<f64>,
        pub image: Option<String>,
        pub last_updated: chrono::DateTime<Utc>,
        pub is_favorite: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PriceHistoryEntry {
        pub timestamp: i64,
        pub price: f64,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PriceHistory {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<mongodb::bson::oid::ObjectId>,
        pub token_id: String,
        pub prices: Vec<PriceHistoryEntry>,
        pub last_updated: chrono::DateTime<Utc>,
    }

    pub fn create_test_token(token_id: &str) -> CryptoToken {
        CryptoToken {
            id: None,
            token_id: token_id.to_string(),
            symbol: token_id.chars().take(3).collect::<String>().to_uppercase(),
            name: format!("Test {}", token_id),
            current_price: 1000.0,
            market_cap: 10000000000.0,
            volume_24h: 1000000000.0,
            price_change_24h: 10.0,
            price_change_percentage_24h: 1.0,
            high_24h: Some(1100.0),
            low_24h: Some(900.0),
            circulating_supply: Some(10000000.0),
            total_supply: Some(21000000.0),
            ath: Some(1500.0),
            ath_change_percentage: Some(-33.33),
            atl: Some(100.0),
            atl_change_percentage: Some(900.0),
            image: Some(format!("https://example.com/{}.png", token_id)),
            last_updated: Utc::now(),
            is_favorite: false,
        }
    }

    pub fn create_test_tokens(count: usize) -> Vec<CryptoToken> {
        (0..count)
            .map(|i| create_test_token(&format!("token{}", i)))
            .collect()
    }

    pub fn create_test_price_history(token_id: &str, days: usize) -> PriceHistory {
        let prices: Vec<PriceHistoryEntry> = (0..days)
            .map(|i| PriceHistoryEntry {
                timestamp: (1000 + i * 86400) as i64,
                price: 1000.0 + (i as f64 * 10.0),
            })
            .collect();

        PriceHistory {
            id: None,
            token_id: token_id.to_string(),
            prices,
            last_updated: Utc::now(),
        }
    }
}
