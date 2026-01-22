use actix_web::{web, HttpResponse, Result};
use mongodb::bson::doc;
use crate::{db::DbClient, models::{FavoriteRequest, TokenStats, PriceHistory, CryptoToken}, crypto_service::CryptoService};
use chrono::{Utc, Duration};
use std::sync::Arc;
use tokio::sync::Mutex;

// Simple in-memory rate limit tracker
lazy_static::lazy_static! {
    static ref LAST_API_CALL: Arc<Mutex<Option<chrono::DateTime<Utc>>>> = Arc::new(Mutex::new(None));
    static ref RATE_LIMITED_UNTIL: Arc<Mutex<Option<chrono::DateTime<Utc>>>> = Arc::new(Mutex::new(None));
}

const MIN_REQUEST_INTERVAL_SECS: i64 = 2; // Minimum 2 seconds between API calls
const RATE_LIMIT_BACKOFF_SECS: i64 = 60; // Wait 60 seconds after rate limit

async fn can_make_api_call() -> bool {
    let rate_limited = RATE_LIMITED_UNTIL.lock().await;
    if let Some(until) = *rate_limited {
        if Utc::now() < until {
            log::info!("Rate limited, waiting until {}", until);
            return false;
        }
    }
    drop(rate_limited);

    let last_call = LAST_API_CALL.lock().await;
    if let Some(last) = *last_call {
        if Utc::now() - last < Duration::seconds(MIN_REQUEST_INTERVAL_SECS) {
            return false;
        }
    }
    true
}

async fn record_api_call() {
    let mut last_call = LAST_API_CALL.lock().await;
    *last_call = Some(Utc::now());
}

async fn record_rate_limit() {
    let mut rate_limited = RATE_LIMITED_UNTIL.lock().await;
    *rate_limited = Some(Utc::now() + Duration::seconds(RATE_LIMIT_BACKOFF_SECS));
    log::warn!("Rate limited! Backing off for {} seconds", RATE_LIMIT_BACKOFF_SECS);
}

async fn save_tokens_to_cache(collection: &mongodb::Collection<CryptoToken>, tokens: &[CryptoToken]) {
    for token in tokens {
        let filter = doc! { "token_id": &token.token_id };
        let update = doc! {
            "$set": {
                "token_id": &token.token_id,
                "symbol": &token.symbol,
                "name": &token.name,
                "current_price": token.current_price,
                "market_cap": token.market_cap,
                "volume_24h": token.volume_24h,
                "price_change_24h": token.price_change_24h,
                "price_change_percentage_24h": token.price_change_percentage_24h,
                "high_24h": token.high_24h,
                "low_24h": token.low_24h,
                "circulating_supply": token.circulating_supply,
                "total_supply": token.total_supply,
                "ath": token.ath,
                "ath_change_percentage": token.ath_change_percentage,
                "atl": token.atl,
                "atl_change_percentage": token.atl_change_percentage,
                "image": &token.image,
                "last_updated": Utc::now(),
            },
            "$setOnInsert": {
                "is_favorite": false,
            }
        };
        
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();
            
        let _ = collection.update_one(filter, update, options).await;
    }
}

async fn get_cached_tokens(collection: &mongodb::Collection<CryptoToken>) -> Vec<CryptoToken> {
    let mut cached_tokens = Vec::new();
    
    if let Ok(mut cursor) = collection.find(None, None).await {
        use futures::stream::StreamExt;
        while let Some(result) = cursor.next().await {
            if let Ok(token) = result {
                cached_tokens.push(token);
            }
        }
    }
    
    // Sort by market cap descending
    cached_tokens.sort_by(|a, b| b.market_cap.partial_cmp(&a.market_cap).unwrap_or(std::cmp::Ordering::Equal));
    cached_tokens
}

pub async fn get_tokens(
    db: web::Data<DbClient>,
    crypto_service: web::Data<CryptoService>,
) -> Result<HttpResponse> {
    let collection = db.get_tokens_collection();
    
    // Get cached tokens first
    let cached_tokens = get_cached_tokens(&collection).await;
    
    // Check if we should try to refresh from API
    if can_make_api_call().await {
        record_api_call().await;
        
        match crypto_service.fetch_top_tokens(100).await {
            Ok(tokens) if !tokens.is_empty() => {
                log::info!("Successfully fetched {} tokens from API", tokens.len());
                
                // Save to cache in background, but return tokens immediately
                let save_collection = collection.clone();
                let tokens_to_save = tokens.clone();
                tokio::spawn(async move {
                    save_tokens_to_cache(&save_collection, &tokens_to_save).await;
                    log::info!("Saved {} tokens to cache", tokens_to_save.len());
                });
                
                // Return the fetched tokens directly
                return Ok(HttpResponse::Ok().json(tokens));
            }
            Ok(_) => {
                log::warn!("API returned empty result");
            }
            Err(e) => {
                let error_msg = e.to_string().to_lowercase();
                if error_msg.contains("429") || error_msg.contains("rate") {
                    record_rate_limit().await;
                }
                log::error!("API error: {}", e);
            }
        }
    }
    
    // Return cached data if available
    if !cached_tokens.is_empty() {
        log::info!("Returning {} cached tokens", cached_tokens.len());
        return Ok(HttpResponse::Ok().json(cached_tokens));
    }
    
    // No cached data and can't fetch - return error with retry hint
    Ok(HttpResponse::ServiceUnavailable().json(doc! {
        "error": "Data temporarily unavailable. Please try again in a moment.",
        "retry_after": 60
    }))
}

pub async fn get_token(
    db: web::Data<DbClient>,
    crypto_service: web::Data<CryptoService>,
    token_id: web::Path<String>,
) -> Result<HttpResponse> {
    let collection = db.get_tokens_collection();
    
    // Try cached first
    if let Ok(Some(token)) = collection.find_one(doc! { "token_id": token_id.as_str() }, None).await {
        return Ok(HttpResponse::Ok().json(token));
    }
    
    // Try API if not rate limited
    if can_make_api_call().await {
        record_api_call().await;
        
        match crypto_service.fetch_token_details(&token_id).await {
            Ok(token) => {
                save_tokens_to_cache(&collection, &[token.clone()]).await;
                return Ok(HttpResponse::Ok().json(token));
            }
            Err(e) => {
                let error_msg = e.to_string().to_lowercase();
                if error_msg.contains("429") || error_msg.contains("rate") {
                    record_rate_limit().await;
                }
                log::error!("Error fetching token details: {}", e);
            }
        }
    }
    
    Ok(HttpResponse::NotFound().json(doc! {
        "error": "Token not found"
    }))
}

pub async fn toggle_favorite(
    db: web::Data<DbClient>,
    req: web::Json<FavoriteRequest>,
) -> Result<HttpResponse> {
    let collection = db.get_tokens_collection();
    
    // First, get current token to toggle favorite
    let filter = doc! { "token_id": &req.token_id };
    
    match collection.find_one(filter.clone(), None).await {
        Ok(Some(token)) => {
            let new_favorite = !token.is_favorite;
            let update = doc! {
                "$set": {
                    "is_favorite": new_favorite
                }
            };
            
            match collection.update_one(filter.clone(), update, None).await {
                Ok(_) => {
                    match collection.find_one(filter, None).await {
                        Ok(Some(token)) => Ok(HttpResponse::Ok().json(token)),
                        _ => Ok(HttpResponse::Ok().json(doc! {
                            "message": "Favorite updated successfully"
                        })),
                    }
                }
                Err(e) => {
                    log::error!("Failed to update favorite: {}", e);
                    Ok(HttpResponse::InternalServerError().json(doc! {
                        "error": "Failed to update favorite"
                    }))
                }
            }
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(doc! {
                "error": "Token not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to find token: {}", e);
            Ok(HttpResponse::InternalServerError().json(doc! {
                "error": "Database error"
            }))
        }
    }
}

pub async fn get_favorites(db: web::Data<DbClient>) -> Result<HttpResponse> {
    let collection = db.get_tokens_collection();
    
    match collection.find(doc! { "is_favorite": true }, None).await {
        Ok(mut cursor) => {
            let mut favorites = Vec::new();
            use futures::stream::StreamExt;
            
            while let Some(result) = cursor.next().await {
                if let Ok(token) = result {
                    favorites.push(token);
                }
            }
            
            Ok(HttpResponse::Ok().json(favorites))
        }
        Err(e) => {
            log::error!("Error fetching favorites: {}", e);
            Ok(HttpResponse::InternalServerError().json(doc! {
                "error": format!("Database error: {}", e)
            }))
        }
    }
}

pub async fn search_tokens(
    db: web::Data<DbClient>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse> {
    let search_query = query.get("q").map(|s| s.as_str()).unwrap_or("");
    
    if search_query.is_empty() {
        return Ok(HttpResponse::BadRequest().json(doc! {
            "error": "Search query is required"
        }));
    }

    let collection = db.get_tokens_collection();
    let search_lower = search_query.to_lowercase();
    
    // Search in cached data instead of making API call
    let cached_tokens = get_cached_tokens(&collection).await;
    
    let filtered: Vec<CryptoToken> = cached_tokens
        .into_iter()
        .filter(|t| {
            t.name.to_lowercase().contains(&search_lower) ||
            t.symbol.to_lowercase().contains(&search_lower) ||
            t.token_id.to_lowercase().contains(&search_lower)
        })
        .collect();
    
    Ok(HttpResponse::Ok().json(filtered))
}

pub async fn get_historical_data(
    crypto_service: web::Data<CryptoService>,
    db: web::Data<DbClient>,
    path: web::Path<(String, u32)>,
) -> Result<HttpResponse> {
    let (token_id, days) = path.into_inner();
    
    // Check rate limit before making API call
    if !can_make_api_call().await {
        // Try to return cached historical data
        let collection = db.get_history_collection();
        let filter = doc! { 
            "token_id": &token_id,
        };
        
        if let Ok(Some(history)) = collection.find_one(filter, None).await {
            log::info!("Returning cached historical data for {}", token_id);
            
            // Convert back to API format
            let response = crate::models::CoinGeckoHistoricalData {
                prices: history.prices.iter().map(|(t, p)| vec![*t as f64, *p]).collect(),
                market_caps: history.market_caps.iter().map(|(t, p)| vec![*t as f64, *p]).collect(),
                total_volumes: history.total_volumes.iter().map(|(t, p)| vec![*t as f64, *p]).collect(),
            };
            
            return Ok(HttpResponse::Ok().json(response));
        }
        
        return Ok(HttpResponse::ServiceUnavailable().json(doc! {
            "error": "Historical data temporarily unavailable. Please try again shortly.",
            "retry_after": 30
        }));
    }
    
    record_api_call().await;
    
    match crypto_service.fetch_historical_data(&token_id, days).await {
        Ok(data) => {
            // Cache the historical data
            let history = PriceHistory {
                id: None,
                token_id: token_id.clone(),
                symbol: token_id.clone(),
                prices: data.prices.iter().map(|p| (p[0] as i64, p[1])).collect(),
                market_caps: data.market_caps.iter().map(|p| (p[0] as i64, p[1])).collect(),
                total_volumes: data.total_volumes.iter().map(|p| (p[0] as i64, p[1])).collect(),
                timestamp: Utc::now(),
            };

            let collection = db.get_history_collection();
            
            // Upsert instead of insert to prevent duplicates
            let filter = doc! { "token_id": &token_id };
            let update = doc! {
                "$set": {
                    "token_id": &history.token_id,
                    "symbol": &history.symbol,
                    "prices": &history.prices.iter().map(|(t, p)| doc! { "t": *t, "p": *p }).collect::<Vec<_>>(),
                    "timestamp": Utc::now(),
                }
            };
            let options = mongodb::options::UpdateOptions::builder().upsert(true).build();
            let _ = collection.update_one(filter, update, options).await;
            
            Ok(HttpResponse::Ok().json(data))
        }
        Err(e) => {
            let error_msg = e.to_string().to_lowercase();
            if error_msg.contains("429") || error_msg.contains("rate") {
                record_rate_limit().await;
            }
            log::error!("Error fetching historical data: {}", e);
            
            Ok(HttpResponse::ServiceUnavailable().json(doc! {
                "error": "Failed to fetch historical data. Please try again shortly.",
                "retry_after": 30
            }))
        }
    }
}

pub async fn get_stats(db: web::Data<DbClient>) -> Result<HttpResponse> {
    let collection = db.get_tokens_collection();
    let tokens = get_cached_tokens(&collection).await;

    if tokens.is_empty() {
        return Ok(HttpResponse::Ok().json(TokenStats {
            total_tokens: 0,
            total_market_cap: 0.0,
            total_volume_24h: 0.0,
            avg_price_change_24h: 0.0,
            biggest_gainer: None,
            biggest_loser: None,
        }));
    }

    let total_market_cap: f64 = tokens.iter().map(|t| t.market_cap).sum();
    let total_volume_24h: f64 = tokens.iter().map(|t| t.volume_24h).sum();
    let avg_price_change_24h: f64 = 
        tokens.iter().map(|t| t.price_change_percentage_24h).sum::<f64>() / tokens.len() as f64;

    let biggest_gainer = tokens.iter()
        .max_by(|a, b| a.price_change_percentage_24h.partial_cmp(&b.price_change_percentage_24h).unwrap())
        .cloned();

    let biggest_loser = tokens.iter()
        .min_by(|a, b| a.price_change_percentage_24h.partial_cmp(&b.price_change_percentage_24h).unwrap())
        .cloned();

    let stats = TokenStats {
        total_tokens: tokens.len(),
        total_market_cap,
        total_volume_24h,
        avg_price_change_24h,
        biggest_gainer,
        biggest_loser,
    };

    Ok(HttpResponse::Ok().json(stats))
}
