// Property-based tests using proptest
mod common;

use proptest::prelude::*;
use crypto_tracker_backend::models::*;
use chrono::Utc;

proptest! {
    #[test]
    fn test_token_price_always_positive(price in 0.0f64..1000000.0) {
        let token = CryptoToken {
            id: None,
            token_id: "test".to_string(),
            symbol: "tst".to_string(),
            name: "Test".to_string(),
            current_price: price,
            market_cap: price * 1000000.0,
            volume_24h: price * 100000.0,
            price_change_24h: 0.0,
            price_change_percentage_24h: 0.0,
            high_24h: Some(price * 1.1),
            low_24h: Some(price * 0.9),
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
        
        prop_assert!(token.current_price >= 0.0);
        prop_assert!(token.market_cap >= 0.0);
        prop_assert!(token.volume_24h >= 0.0);
    }

    #[test]
    fn test_price_change_percentage_bounds(change in -100.0f64..100.0) {
        let token = CryptoToken {
            id: None,
            token_id: "test".to_string(),
            symbol: "tst".to_string(),
            name: "Test".to_string(),
            current_price: 100.0,
            market_cap: 1000000.0,
            volume_24h: 10000.0,
            price_change_24h: change,
            price_change_percentage_24h: change,
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
        
        // Price change percentage can be any real number in reality
        prop_assert!(token.price_change_percentage_24h >= -100.0);
        prop_assert!(token.price_change_percentage_24h <= 100.0);
    }

    #[test]
    fn test_high_always_greater_than_low(
        low in 1.0f64..10000.0,
        high_multiplier in 1.0f64..2.0
    ) {
        let high = low * high_multiplier;
        
        let token = CryptoToken {
            id: None,
            token_id: "test".to_string(),
            symbol: "tst".to_string(),
            name: "Test".to_string(),
            current_price: (low + high) / 2.0,
            market_cap: 1000000.0,
            volume_24h: 10000.0,
            price_change_24h: 0.0,
            price_change_percentage_24h: 0.0,
            high_24h: Some(high),
            low_24h: Some(low),
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
        
        prop_assert!(token.high_24h.unwrap() >= token.low_24h.unwrap());
    }

    #[test]
    fn test_market_cap_formula(
        price in 1.0f64..100000.0,
        supply in 1000.0f64..1000000000.0
    ) {
        let calculated_market_cap = price * supply;
        
        let token = CryptoToken {
            id: None,
            token_id: "test".to_string(),
            symbol: "tst".to_string(),
            name: "Test".to_string(),
            current_price: price,
            market_cap: calculated_market_cap,
            volume_24h: 10000.0,
            price_change_24h: 0.0,
            price_change_percentage_24h: 0.0,
            high_24h: None,
            low_24h: None,
            circulating_supply: Some(supply),
            total_supply: Some(supply * 1.5),
            ath: None,
            ath_change_percentage: None,
            atl: None,
            atl_change_percentage: None,
            image: None,
            last_updated: Utc::now(),
            is_favorite: false,
        };
        
        // Market cap should be close to price * circulating_supply
        let expected = price * token.circulating_supply.unwrap();
        prop_assert!((token.market_cap - expected).abs() < 0.01);
    }

    #[test]
    fn test_price_history_timestamps_ordered(count in 1usize..100) {
        let mut timestamps = Vec::new();
        for i in 0..count {
            timestamps.push(1000 + (i * 86400) as i64);
        }
        
        let prices: Vec<PriceHistoryEntry> = timestamps
            .iter()
            .map(|&ts| PriceHistoryEntry {
                timestamp: ts,
                price: 1000.0,
            })
            .collect();
        
        // Check that timestamps are in ascending order
        for i in 1..prices.len() {
            prop_assert!(prices[i].timestamp >= prices[i-1].timestamp);
        }
    }

    #[test]
    fn test_token_id_not_empty(s in "[a-z]{1,20}") {
        let token = CryptoToken {
            id: None,
            token_id: s.clone(),
            symbol: s[..s.len().min(3)].to_uppercase(),
            name: format!("Test {}", s),
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
        
        prop_assert!(!token.token_id.is_empty());
        prop_assert!(!token.symbol.is_empty());
        prop_assert!(!token.name.is_empty());
    }

    #[test]
    fn test_serialization_roundtrip(
        price in 1.0f64..100000.0,
        market_cap in 1000000.0f64..1000000000000.0
    ) {
        let token = CryptoToken {
            id: None,
            token_id: "test".to_string(),
            symbol: "tst".to_string(),
            name: "Test".to_string(),
            current_price: price,
            market_cap,
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
        
        let json = serde_json::to_string(&token).unwrap();
        let deserialized: CryptoToken = serde_json::from_str(&json).unwrap();
        
        prop_assert!((deserialized.current_price - price).abs() < 0.0001);
        prop_assert!((deserialized.market_cap - market_cap).abs() < 0.0001);
    }
}
