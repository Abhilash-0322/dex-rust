// Integration tests for database operations
mod common;

use mongodb::bson::doc;
use serial_test::serial;
use futures::stream::StreamExt;

#[tokio::test]
#[serial]
async fn test_database_connection() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    
    // Test that we can list collections
    let collections = db.list_collection_names(None).await;
    assert!(collections.is_ok());
    
    common::cleanup_test_db(&db).await;
}

#[tokio::test]
#[serial]
async fn test_insert_and_retrieve_token() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    
    let token = common::mock_data::create_test_token("bitcoin");
    let collection = db.collection::<common::mock_data::CryptoToken>("tokens");
    
    // Insert token
    let insert_result = collection.insert_one(&token, None).await;
    assert!(insert_result.is_ok());
    
    // Retrieve token
    let found_token = collection
        .find_one(doc! { "token_id": "bitcoin" }, None)
        .await
        .unwrap();
    
    assert!(found_token.is_some());
    let found_token = found_token.unwrap();
    assert_eq!(found_token.token_id, "bitcoin");
    assert_eq!(found_token.current_price, 1000.0);
    
    common::cleanup_test_db(&db).await;
}

#[tokio::test]
#[serial]
async fn test_toggle_favorite() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    
    let token = common::mock_data::create_test_token("ethereum");
    let collection = db.collection::<common::mock_data::CryptoToken>("tokens");
    
    collection.insert_one(&token, None).await.unwrap();
    
    // Toggle favorite
    let update_result = collection
        .update_one(
            doc! { "token_id": "ethereum" },
            doc! { "$set": { "is_favorite": true } },
            None
        )
        .await;
    
    assert!(update_result.is_ok());
    assert_eq!(update_result.unwrap().modified_count, 1);
    
    // Verify update
    let updated_token = collection
        .find_one(doc! { "token_id": "ethereum" }, None)
        .await
        .unwrap()
        .unwrap();
    
    assert!(updated_token.is_favorite);
    
    common::cleanup_test_db(&db).await;
}

#[tokio::test]
#[serial]
async fn test_get_favorites() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    
    // Insert favorite and non-favorite tokens
    let mut fav1 = common::mock_data::create_test_token("bitcoin");
    fav1.is_favorite = true;
    
    let mut fav2 = common::mock_data::create_test_token("ethereum");
    fav2.is_favorite = true;
    
    let non_fav = common::mock_data::create_test_token("cardano");
    
    let collection = db.collection::<common::mock_data::CryptoToken>("tokens");
    collection.insert_many(vec![&fav1, &fav2, &non_fav], None).await.unwrap();
    
    // Find only favorites
    let mut cursor = collection
        .find(doc! { "is_favorite": true }, None)
        .await
        .unwrap();
    
    let mut favorites = Vec::new();
    while let Some(result) = cursor.next().await {
        favorites.push(result.unwrap());
    }
    
    assert_eq!(favorites.len(), 2);
    assert!(favorites.iter().all(|t| t.is_favorite));
    
    common::cleanup_test_db(&db).await;
}

#[tokio::test]
#[serial]
async fn test_bulk_operations() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    
    let tokens = common::mock_data::create_test_tokens(10);
    let collection = db.collection::<common::mock_data::CryptoToken>("tokens");
    
    let insert_result = collection.insert_many(&tokens, None).await;
    assert!(insert_result.is_ok());
    assert_eq!(insert_result.unwrap().inserted_ids.len(), 10);
    
    // Count documents
    let count = collection.count_documents(doc! {}, None).await.unwrap();
    assert_eq!(count, 10);
    
    common::cleanup_test_db(&db).await;
}

#[actix_rt::test]
#[serial]
async fn test_get_all_tokens_empty_db() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .route("/api/tokens", web::get().to(get_all_tokens))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/tokens")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    common::cleanup_test_db(&db).await;
}

#[actix_rt::test]
#[serial]
async fn test_toggle_favorite_token() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    // Insert a test token
    let token = common::mock_data::create_test_token("bitcoin");
    let collection = db_client.get_tokens_collection();
    collection.insert_one(&token, None).await.unwrap();
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .route("/api/tokens/favorite", web::post().to(toggle_favorite))
    ).await;
    
    let favorite_req = FavoriteRequest {
        token_id: "bitcoin".to_string(),
    };
    
    let req = test::TestRequest::post()
        .uri("/api/tokens/favorite")
        .set_json(&favorite_req)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    // Verify favorite was toggled
    let updated_token = collection
        .find_one(doc! { "token_id": "bitcoin" }, None)
        .await
        .unwrap()
        .unwrap();
    
    assert!(updated_token.is_favorite);
    
    common::cleanup_test_db(&db).await;
}

#[actix_rt::test]
#[serial]
async fn test_get_favorites_empty() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .route("/api/favorites", web::get().to(get_favorites))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/favorites")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let body: Vec<CryptoToken> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 0);
    
    common::cleanup_test_db(&db).await;
}

#[actix_rt::test]
#[serial]
async fn test_get_favorites_with_data() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    // Insert favorite and non-favorite tokens
    let mut fav_token = common::mock_data::create_test_token("ethereum");
    fav_token.is_favorite = true;
    
    let non_fav_token = common::mock_data::create_test_token("cardano");
    
    let collection = db_client.get_tokens_collection();
    collection.insert_one(&fav_token, None).await.unwrap();
    collection.insert_one(&non_fav_token, None).await.unwrap();
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .route("/api/favorites", web::get().to(get_favorites))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/favorites")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let body: Vec<CryptoToken> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0].token_id, "ethereum");
    assert!(body[0].is_favorite);
    
    common::cleanup_test_db(&db).await;
}

#[actix_rt::test]
#[serial]
async fn test_get_token_by_id_not_found() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .route("/api/tokens/{id}", web::get().to(get_token_by_id))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/tokens/nonexistent")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
    
    common::cleanup_test_db(&db).await;
}

#[actix_rt::test]
#[serial]
async fn test_search_tokens_empty_query() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    // Insert some tokens
    let tokens = common::mock_data::create_test_tokens(5);
    let collection = db_client.get_tokens_collection();
    collection.insert_many(&tokens, None).await.unwrap();
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .route("/api/search", web::get().to(search_tokens))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/search?q=")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let body: Vec<CryptoToken> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 5); // Returns all tokens when query is empty
    
    common::cleanup_test_db(&db).await;
}

#[actix_rt::test]
#[serial]
async fn test_search_tokens_with_query() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    // Insert tokens with specific names
    let mut bitcoin = common::mock_data::create_test_token("bitcoin");
    bitcoin.name = "Bitcoin".to_string();
    
    let mut ethereum = common::mock_data::create_test_token("ethereum");
    ethereum.name = "Ethereum".to_string();
    
    let collection = db_client.get_tokens_collection();
    collection.insert_many(vec![&bitcoin, &ethereum], None).await.unwrap();
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .route("/api/search", web::get().to(search_tokens))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/search?q=bit")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let body: Vec<CryptoToken> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0].token_id, "bitcoin");
    
    common::cleanup_test_db(&db).await;
}

#[actix_rt::test]
#[serial]
async fn test_invalid_favorite_request() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .route("/api/tokens/favorite", web::post().to(toggle_favorite))
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/tokens/favorite")
        .set_payload("{invalid json}")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
    
    common::cleanup_test_db(&db).await;
}
