use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoToken {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
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
    pub last_updated: DateTime<Utc>,
    pub is_favorite: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinGeckoMarket {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub current_price: f64,
    pub market_cap: f64,
    pub market_cap_rank: Option<u32>,
    pub fully_diluted_valuation: Option<f64>,
    pub total_volume: f64,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub price_change_percentage_24h: Option<f64>,
    pub market_cap_change_24h: Option<f64>,
    pub market_cap_change_percentage_24h: Option<f64>,
    pub circulating_supply: Option<f64>,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub ath: Option<f64>,
    pub ath_change_percentage: Option<f64>,
    pub ath_date: Option<String>,
    pub atl: Option<f64>,
    pub atl_change_percentage: Option<f64>,
    pub atl_date: Option<String>,
    pub last_updated: String,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct FavoriteRequest {
    pub token_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceHistory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub token_id: String,
    pub symbol: String,
    pub prices: Vec<(i64, f64)>,
    pub market_caps: Vec<(i64, f64)>,
    pub total_volumes: Vec<(i64, f64)>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinGeckoHistoricalData {
    pub prices: Vec<Vec<f64>>,
    pub market_caps: Vec<Vec<f64>>,
    pub total_volumes: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize)]
pub struct TokenStats {
    pub total_tokens: usize,
    pub total_market_cap: f64,
    pub total_volume_24h: f64,
    pub avg_price_change_24h: f64,
    pub biggest_gainer: Option<CryptoToken>,
    pub biggest_loser: Option<CryptoToken>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketStats {
    pub total_market_cap: f64,
    pub total_volume_24h: f64,
    pub bitcoin_dominance: f64,
    pub top_gainer: Option<TokenChange>,
    pub top_loser: Option<TokenChange>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenChange {
    pub token_id: String,
    pub name: String,
    pub symbol: String,
    pub change_percentage: f64,
    pub current_price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceHistoryEntry {
    pub timestamp: i64,
    pub price: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_crypto_token_creation() {
        let token = CryptoToken {
            id: None,
            token_id: "bitcoin".to_string(),
            symbol: "btc".to_string(),
            name: "Bitcoin".to_string(),
            current_price: 50000.0,
            market_cap: 1000000000000.0,
            volume_24h: 50000000000.0,
            price_change_24h: 1000.0,
            price_change_percentage_24h: 2.5,
            high_24h: Some(51000.0),
            low_24h: Some(49000.0),
            circulating_supply: Some(19000000.0),
            total_supply: Some(21000000.0),
            ath: Some(69000.0),
            ath_change_percentage: Some(-27.5),
            atl: Some(67.81),
            atl_change_percentage: Some(73600.0),
            image: Some("https://example.com/bitcoin.png".to_string()),
            last_updated: Utc::now(),
            is_favorite: false,
        };

        assert_eq!(token.token_id, "bitcoin");
        assert_eq!(token.current_price, 50000.0);
        assert!(!token.is_favorite);
    }

    #[test]
    fn test_favorite_request_serialization() {
        let request = FavoriteRequest {
            token_id: "ethereum".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("ethereum"));
        
        let deserialized: FavoriteRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.token_id, "ethereum");
    }

    #[test]
    fn test_market_stats_with_gainers_losers() {
        let stats = MarketStats {
            total_market_cap: 2000000000000.0,
            total_volume_24h: 100000000000.0,
            bitcoin_dominance: 45.5,
            top_gainer: Some(TokenChange {
                token_id: "winner".to_string(),
                name: "Winner".to_string(),
                symbol: "win".to_string(),
                change_percentage: 50.0,
                current_price: 10.0,
            }),
            top_loser: Some(TokenChange {
                token_id: "loser".to_string(),
                name: "Loser".to_string(),
                symbol: "lose".to_string(),
                change_percentage: -30.0,
                current_price: 5.0,
            }),
        };

        assert!(stats.top_gainer.is_some());
        assert!(stats.top_loser.is_some());
        assert!(stats.top_gainer.as_ref().unwrap().change_percentage > 0.0);
        assert!(stats.top_loser.as_ref().unwrap().change_percentage < 0.0);
    }
}
