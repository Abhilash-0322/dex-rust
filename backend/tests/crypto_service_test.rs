// Tests for CryptoService with mock HTTP server
mod common;

use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path_regex};

// Mock HTTP client tests
#[tokio::test]
async fn test_mock_api_success() {
    common::init_test_logger();
    
    let mock_server = MockServer::start().await;
    
    let response_body = r#"[
        {
            "id": "bitcoin",
            "symbol": "btc",
            "name": "Bitcoin",
            "image": "https://example.com/btc.png",
            "current_price": 50000.0,
            "market_cap": 1000000000000.0,
            "total_volume": 50000000000.0,
            "price_change_24h": 1000.0,
            "price_change_percentage_24h": 2.0,
            "high_24h": 51000.0,
            "low_24h": 49000.0,
            "circulating_supply": 19000000.0,
            "total_supply": 21000000.0,
            "ath": 69000.0,
            "ath_change_percentage": -27.5,
            "atl": 67.81,
            "atl_change_percentage": 73600.0
        }
    ]"#;
    
    Mock::given(method("GET"))
        .and(path_regex("/.*"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;
    
    // Test HTTP request
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/coins/markets", mock_server.uri()))
        .send()
        .await;
    
    assert!(response.is_ok());
    assert!(response.unwrap().status().is_success());
}

#[tokio::test]
async fn test_mock_api_rate_limit() {
    common::init_test_logger();
    
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(429)) // Rate limit error
        .mount(&mock_server)
        .await;
    
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/test", mock_server.uri()))
        .send()
        .await;
    
    assert!(response.is_ok());
    let status = response.unwrap().status();
    assert_eq!(status, 429);
}

#[tokio::test]
async fn test_mock_api_timeout() {
    common::init_test_logger();
    
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(std::time::Duration::from_secs(20))
        )
        .mount(&mock_server)
        .await;
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(1))
        .build()
        .unwrap();
    
    let response = client
        .get(&format!("{}/test", mock_server.uri()))
        .send()
        .await;
    
    // Should timeout
    assert!(response.is_err());
}

#[tokio::test]
async fn test_crypto_service_fetch_top_tokens_success() {
    common::init_test_logger();
    
    let mock_server = MockServer::start().await;
    
    let response_body = r#"[
        {
            "id": "bitcoin",
            "symbol": "btc",
            "name": "Bitcoin",
            "image": "https://example.com/btc.png",
            "current_price": 50000.0,
            "market_cap": 1000000000000.0,
            "total_volume": 50000000000.0,
            "price_change_24h": 1000.0,
            "price_change_percentage_24h": 2.0,
            "high_24h": 51000.0,
            "low_24h": 49000.0,
            "circulating_supply": 19000000.0,
            "total_supply": 21000000.0,
            "ath": 69000.0,
            "ath_change_percentage": -27.5,
            "atl": 67.81,
            "atl_change_percentage": 73600.0
        }
    ]"#;
    
    Mock::given(method("GET"))
        .and(path("/coins/markets"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;
    
    let service = CryptoService::new(mock_server.uri());
    let result = service.fetch_top_tokens(1).await;
    
    assert!(result.is_ok());
    let tokens = result.unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_id, "bitcoin");
    assert_eq!(tokens[0].symbol, "btc");
    assert_eq!(tokens[0].current_price, 50000.0);
}

#[tokio::test]
async fn test_crypto_service_fetch_top_tokens_api_error() {
    common::init_test_logger();
    
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/coins/markets"))
        .respond_with(ResponseTemplate::new(429)) // Rate limit error
        .mount(&mock_server)
        .await;
    
    let service = CryptoService::new(mock_server.uri());
    let result = service.fetch_top_tokens(1).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_crypto_service_fetch_top_tokens_invalid_json() {
    common::init_test_logger();
    
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/coins/markets"))
        .respond_with(ResponseTemplate::new(200).set_body_string("{invalid json}"))
        .mount(&mock_server)
        .await;
    
    let service = CryptoService::new(mock_server.uri());
    let result = service.fetch_top_tokens(1).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_crypto_service_fetch_historical_data_success() {
    common::init_test_logger();
    
    let mock_server = MockServer::start().await;
    
    let response_body = r#"{
        "prices": [
            [1640000000000, 47000.0],
            [1640086400000, 48000.0],
            [1640172800000, 49000.0]
        ]
    }"#;
    
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;
    
    let service = CryptoService::new(mock_server.uri());
    let result = service.fetch_historical_data("bitcoin", 7).await;
    
    assert!(result.is_ok());
    let history = result.unwrap();
    assert_eq!(history.token_id, "bitcoin");
    assert_eq!(history.prices.len(), 3);
    assert_eq!(history.prices[0].price, 47000.0);
}

#[tokio::test]
async fn test_crypto_service_fetch_historical_data_empty_prices() {
    common::init_test_logger();
    
    let mock_server = MockServer::start().await;
    
    let response_body = r#"{"prices": []}"#;
    
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;
    
    let service = CryptoService::new(mock_server.uri());
    let result = service.fetch_historical_data("bitcoin", 7).await;
    
    assert!(result.is_ok());
    let history = result.unwrap();
    assert_eq!(history.prices.len(), 0);
}

#[tokio::test]
async fn test_crypto_service_timeout() {
    common::init_test_logger();
    
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(std::time::Duration::from_secs(20))
        )
        .mount(&mock_server)
        .await;
    
    let service = CryptoService::new(mock_server.uri());
    let result = service.fetch_top_tokens(1).await;
    
    // Should timeout and return an error
    assert!(result.is_err());
}

#[tokio::test]
async fn test_crypto_service_handles_null_values() {
    common::init_test_logger();
    
    let mock_server = MockServer::start().await;
    
    let response_body = r#"[
        {
            "id": "test-coin",
            "symbol": "tst",
            "name": "Test Coin",
            "image": "https://example.com/test.png",
            "current_price": 1.0,
            "market_cap": 1000000.0,
            "total_volume": 10000.0,
            "price_change_24h": null,
            "price_change_percentage_24h": null,
            "high_24h": null,
            "low_24h": null,
            "circulating_supply": null,
            "total_supply": null,
            "ath": null,
            "ath_change_percentage": null,
            "atl": null,
            "atl_change_percentage": null
        }
    ]"#;
    
    Mock::given(method("GET"))
        .and(path("/coins/markets"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;
    
    let service = CryptoService::new(mock_server.uri());
    let result = service.fetch_top_tokens(1).await;
    
    assert!(result.is_ok());
    let tokens = result.unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].price_change_24h, 0.0); // Should default to 0
    assert!(tokens[0].high_24h.is_none());
}
