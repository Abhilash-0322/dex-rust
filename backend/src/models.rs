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



#[derive(Debug, Deserialize)]
pub struct FavoriteRequest {
    pub token_id: String,
    pub is_favorite: bool,
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
