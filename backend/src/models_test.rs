// Unit tests for models
#[cfg(test)]
mod tests {
    use super::super::*;
    use chrono::Utc;
    use serde_json;

    #[test]
    fn test_crypto_token_serialization() {
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

        let json = serde_json::to_string(&token).expect("Failed to serialize");
        assert!(json.contains("bitcoin"));
        assert!(json.contains("Bitcoin"));
        
        let deserialized: CryptoToken = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.token_id, "bitcoin");
        assert_eq!(deserialized.current_price, 50000.0);
    }

    #[test]
    fn test_crypto_token_with_missing_optional_fields() {
        let token = CryptoToken {
            id: None,
            token_id: "test-coin".to_string(),
            symbol: "tst".to_string(),
            name: "Test Coin".to_string(),
            current_price: 1.0,
            market_cap: 1000000.0,
            volume_24h: 500000.0,
            price_change_24h: 0.1,
            price_change_percentage_24h: 10.0,
            high_24h: None,
            low_24h: None,
            circulating_supply: None,
            total_supply: None,
            ath: None,
            ath_change_percentage: None,
            atl: None,
            atl_change_percentage: None,
            image: None,
            last_updated: Utc::now(),
            is_favorite: false,
        };

        let json = serde_json::to_string(&token).expect("Failed to serialize");
        let deserialized: CryptoToken = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(deserialized.token_id, "test-coin");
        assert!(deserialized.high_24h.is_none());
        assert!(deserialized.image.is_none());
    }

    #[test]
    fn test_coingecko_market_deserialization() {
        let json = r#"{
            "id": "ethereum",
            "symbol": "eth",
            "name": "Ethereum",
            "image": "https://example.com/eth.png",
            "current_price": 3000.0,
            "market_cap": 360000000000.0,
            "market_cap_rank": 2,
            "total_volume": 15000000000.0,
            "high_24h": 3100.0,
            "low_24h": 2900.0,
            "price_change_24h": 50.0,
            "price_change_percentage_24h": 1.7,
            "circulating_supply": 120000000.0,
            "total_supply": 120000000.0,
            "ath": 4878.26,
            "ath_change_percentage": -38.5,
            "atl": 0.432979,
            "atl_change_percentage": 693000.0
        }"#;

        let market: CoinGeckoMarket = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(market.id, "ethereum");
        assert_eq!(market.symbol, "eth");
        assert_eq!(market.current_price, 3000.0);
        assert_eq!(market.market_cap_rank, Some(2));
    }

    #[test]
    fn test_price_history_entry_serialization() {
        let entry = PriceHistoryEntry {
            timestamp: 1234567890,
            price: 45000.0,
        };

        let json = serde_json::to_string(&entry).expect("Failed to serialize");
        assert!(json.contains("1234567890"));
        assert!(json.contains("45000"));
    }

    #[test]
    fn test_price_history_serialization() {
        let history = PriceHistory {
            id: None,
            token_id: "bitcoin".to_string(),
            prices: vec![
                PriceHistoryEntry { timestamp: 1000, price: 50000.0 },
                PriceHistoryEntry { timestamp: 2000, price: 51000.0 },
            ],
            last_updated: Utc::now(),
        };

        let json = serde_json::to_string(&history).expect("Failed to serialize");
        let deserialized: PriceHistory = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(deserialized.token_id, "bitcoin");
        assert_eq!(deserialized.prices.len(), 2);
        assert_eq!(deserialized.prices[0].price, 50000.0);
    }

    #[test]
    fn test_favorite_request_serialization() {
        let request = FavoriteRequest {
            token_id: "cardano".to_string(),
        };

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(json.contains("cardano"));
    }

    #[test]
    fn test_market_stats_serialization() {
        let stats = MarketStats {
            total_market_cap: 2000000000000.0,
            total_volume_24h: 100000000000.0,
            bitcoin_dominance: 45.5,
            top_gainer: Some(TokenChange {
                token_id: "winner-coin".to_string(),
                name: "Winner Coin".to_string(),
                symbol: "win".to_string(),
                change_percentage: 50.0,
                current_price: 10.0,
            }),
            top_loser: Some(TokenChange {
                token_id: "loser-coin".to_string(),
                name: "Loser Coin".to_string(),
                symbol: "lose".to_string(),
                change_percentage: -30.0,
                current_price: 5.0,
            }),
        };

        let json = serde_json::to_string(&stats).expect("Failed to serialize");
        assert!(json.contains("total_market_cap"));
        assert!(json.contains("bitcoin_dominance"));
        
        let deserialized: MarketStats = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.bitcoin_dominance, 45.5);
    }

    #[test]
    fn test_token_change_with_negative_change() {
        let change = TokenChange {
            token_id: "test".to_string(),
            name: "Test".to_string(),
            symbol: "tst".to_string(),
            change_percentage: -15.5,
            current_price: 100.0,
        };

        assert!(change.change_percentage < 0.0);
        
        let json = serde_json::to_string(&change).expect("Failed to serialize");
        let deserialized: TokenChange = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.change_percentage, -15.5);
    }

    #[test]
    fn test_crypto_token_default_favorite_false() {
        let token = CryptoToken {
            id: None,
            token_id: "test".to_string(),
            symbol: "tst".to_string(),
            name: "Test".to_string(),
            current_price: 1.0,
            market_cap: 1000000.0,
            volume_24h: 10000.0,
            price_change_24h: 0.0,
            price_change_percentage_24h: 0.0,
            high_24h: None,
            low_24h: None,
            circulating_supply: None,
            total_supply: None,
            ath: None,
            ath_change_percentage: None,
            atl: None,
            atl_change_percentage: None,
            image: None,
            last_updated: Utc::now(),
            is_favorite: false,
        };

        assert!(!token.is_favorite);
    }
}
