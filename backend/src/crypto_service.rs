use reqwest::Client;
use crate::models::{CoinGeckoMarket, CoinGeckoHistoricalData, CryptoToken};
use chrono::Utc;

#[derive(Clone)]
pub struct CryptoService {
    client: Client,
    base_url: String,
}

impl CryptoService {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .user_agent("CryptoTracker/1.0 (Educational Project)")
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .unwrap_or_else(|_| Client::new());
            
        Self {
            client,
            base_url,
        }
    }

    pub async fn fetch_top_tokens(&self, limit: u32) -> Result<Vec<CryptoToken>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/coins/markets?vs_currency=usd&order=market_cap_desc&per_page={}&page=1&sparkline=false&price_change_percentage=24h",
            self.base_url, limit
        );

        log::info!("Fetching tokens from: {}", url);
        
        let response = self.client
            .get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;
        
        log::debug!("API Response status: {}, body length: {}", status, text.len());
        
        if !status.is_success() {
            log::error!("API error: status={}, body={}", status, text);
            return Err(format!("API returned error: {}", status).into());
        }
        
        let markets: Vec<CoinGeckoMarket> = match serde_json::from_str(&text) {
            Ok(m) => m,
            Err(e) => {
                log::error!("Failed to parse API response: {}. Response: {}", e, &text[..text.len().min(500)]);
                return Err(format!("Failed to parse API response: {}", e).into());
            }
        };

        let tokens = markets
            .into_iter()
            .map(|market| CryptoToken {
                id: None,
                token_id: market.id,
                symbol: market.symbol,
                name: market.name,
                current_price: market.current_price,
                market_cap: market.market_cap,
                volume_24h: market.total_volume,
                price_change_24h: market.price_change_24h.unwrap_or(0.0),
                price_change_percentage_24h: market.price_change_percentage_24h.unwrap_or(0.0),
                high_24h: market.high_24h,
                low_24h: market.low_24h,
                circulating_supply: market.circulating_supply,
                total_supply: market.total_supply,
                ath: market.ath,
                ath_change_percentage: market.ath_change_percentage,
                atl: market.atl,
                atl_change_percentage: market.atl_change_percentage,
                image: Some(market.image),
                last_updated: Utc::now(),
                is_favorite: false,
            })
            .collect();

        Ok(tokens)
    }

    pub async fn fetch_token_details(&self, token_id: &str) -> Result<CryptoToken, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/coins/markets?vs_currency=usd&ids={}&order=market_cap_desc&sparkline=false&price_change_percentage=24h",
            self.base_url, token_id
        );

        let response = self.client
            .get(&url)
            .send()
            .await?;

        let mut markets: Vec<CoinGeckoMarket> = response.json().await?;

        if let Some(market) = markets.pop() {
            Ok(CryptoToken {
                id: None,
                token_id: market.id,
                symbol: market.symbol,
                name: market.name,
                current_price: market.current_price,
                market_cap: market.market_cap,
                volume_24h: market.total_volume,
                price_change_24h: market.price_change_24h.unwrap_or(0.0),
                price_change_percentage_24h: market.price_change_percentage_24h.unwrap_or(0.0),
                high_24h: market.high_24h,
                low_24h: market.low_24h,
                circulating_supply: market.circulating_supply,
                total_supply: market.total_supply,
                ath: market.ath,
                ath_change_percentage: market.ath_change_percentage,
                atl: market.atl,
                atl_change_percentage: market.atl_change_percentage,
                image: Some(market.image),
                last_updated: Utc::now(),
                is_favorite: false,
            })
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Token not found")))
        }
    }

    pub async fn fetch_historical_data(
        &self,
        token_id: &str,
        days: u32,
    ) -> Result<CoinGeckoHistoricalData, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/coins/{}/market_chart?vs_currency=usd&days={}",
            self.base_url, token_id, days
        );

        let response = self.client
            .get(&url)
            .send()
            .await?;

        let data = response.json().await?;
        Ok(data)
    }

    pub async fn search_tokens(&self, query: &str) -> Result<Vec<CryptoToken>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=50&page=1&sparkline=false",
            self.base_url
        );

        let response = self.client
            .get(&url)
            .send()
            .await?;

        let markets: Vec<CoinGeckoMarket> = response.json().await?;

        let query_lower = query.to_lowercase();
        let tokens: Vec<CryptoToken> = markets
            .into_iter()
            .filter(|market| {
                market.name.to_lowercase().contains(&query_lower)
                    || market.symbol.to_lowercase().contains(&query_lower)
                    || market.id.to_lowercase().contains(&query_lower)
            })
            .map(|market| CryptoToken {
                id: None,
                token_id: market.id,
                symbol: market.symbol,
                name: market.name,
                current_price: market.current_price,
                market_cap: market.market_cap,
                volume_24h: market.total_volume,
                price_change_24h: market.price_change_24h.unwrap_or(0.0),
                price_change_percentage_24h: market.price_change_percentage_24h.unwrap_or(0.0),
                high_24h: market.high_24h,
                low_24h: market.low_24h,
                circulating_supply: market.circulating_supply,
                total_supply: market.total_supply,
                ath: market.ath,
                ath_change_percentage: market.ath_change_percentage,
                atl: market.atl,
                atl_change_percentage: market.atl_change_percentage,
                image: Some(market.image),
                last_updated: Utc::now(),
                is_favorite: false,
            })
            .collect();

        Ok(tokens)
    }
}
