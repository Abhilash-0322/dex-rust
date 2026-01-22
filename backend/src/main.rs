mod models;
mod handlers;
mod db;
mod crypto_service;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use crypto_service::CryptoService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let coingecko_api = env::var("COINGECKO_API_URL")
        .unwrap_or_else(|_| "https://api.coingecko.com/api/v3".to_string());

    log::info!("Connecting to MongoDB at {}", mongodb_uri);
    let db_client = db::init_db(&mongodb_uri, &database_name).await;

    log::info!("Initializing CoinGecko API client");
    let crypto_service = CryptoService::new(coingecko_api);

    log::info!("Starting server at {}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .app_data(web::Data::new(crypto_service.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .route("/tokens", web::get().to(handlers::get_tokens))
                    .route("/tokens/{id}", web::get().to(handlers::get_token))
                    .route("/tokens/favorite", web::post().to(handlers::toggle_favorite))
                    .route("/favorites", web::get().to(handlers::get_favorites))
                    .route("/search", web::get().to(handlers::search_tokens))
                    .route("/history/{id}/{days}", web::get().to(handlers::get_historical_data))
                    .route("/stats", web::get().to(handlers::get_stats))
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
