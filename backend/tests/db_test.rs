// Database operation tests
mod common;

use mongodb::bson::doc;
use crypto_tracker_backend::{db::DbClient, models::*};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_db_connection() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    
    // Test that we can list collections (even if empty)
    let collections = db.list_collection_names(None).await;
    assert!(collections.is_ok());
    
    common::cleanup_test_db(&db).await;
}

#[tokio::test]
#[serial]
async fn test_insert_and_retrieve_token() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    let token = common::mock_data::create_test_token("bitcoin");
    let collection = db_client.get_tokens_collection();
    
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
async fn test_update_token() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    let token = common::mock_data::create_test_token("ethereum");
    let collection = db_client.get_tokens_collection();
    
    collection.insert_one(&token, None).await.unwrap();
    
    // Update price
    let update_result = collection
        .update_one(
            doc! { "token_id": "ethereum" },
            doc! { "$set": { "current_price": 2000.0 } },
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
    
    assert_eq!(updated_token.current_price, 2000.0);
    
    common::cleanup_test_db(&db).await;
}

#[tokio::test]
#[serial]
async fn test_delete_token() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    let token = common::mock_data::create_test_token("cardano");
    let collection = db_client.get_tokens_collection();
    
    collection.insert_one(&token, None).await.unwrap();
    
    // Delete token
    let delete_result = collection
        .delete_one(doc! { "token_id": "cardano" }, None)
        .await;
    
    assert!(delete_result.is_ok());
    assert_eq!(delete_result.unwrap().deleted_count, 1);
    
    // Verify deletion
    let found_token = collection
        .find_one(doc! { "token_id": "cardano" }, None)
        .await
        .unwrap();
    
    assert!(found_token.is_none());
    
    common::cleanup_test_db(&db).await;
}

#[tokio::test]
#[serial]
async fn test_bulk_insert_tokens() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    let tokens = common::mock_data::create_test_tokens(10);
    let collection = db_client.get_tokens_collection();
    
    let insert_result = collection.insert_many(&tokens, None).await;
    assert!(insert_result.is_ok());
    assert_eq!(insert_result.unwrap().inserted_ids.len(), 10);
    
    // Count documents
    let count = collection.count_documents(doc! {}, None).await.unwrap();
    assert_eq!(count, 10);
    
    common::cleanup_test_db(&db).await;
}

#[tokio::test]
#[serial]
async fn test_find_favorites() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    let mut fav1 = common::mock_data::create_test_token("bitcoin");
    fav1.is_favorite = true;
    
    let mut fav2 = common::mock_data::create_test_token("ethereum");
    fav2.is_favorite = true;
    
    let non_fav = common::mock_data::create_test_token("cardano");
    
    let collection = db_client.get_tokens_collection();
    collection.insert_many(vec![&fav1, &fav2, &non_fav], None).await.unwrap();
    
    // Find only favorites
    let mut cursor = collection
        .find(doc! { "is_favorite": true }, None)
        .await
        .unwrap();
    
    use futures::stream::StreamExt;
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
async fn test_insert_price_history() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    let history = common::mock_data::create_test_price_history("bitcoin", 30);
    let collection = db_client.get_history_collection();
    
    let insert_result = collection.insert_one(&history, None).await;
    assert!(insert_result.is_ok());
    
    // Retrieve history
    let found_history = collection
        .find_one(doc! { "token_id": "bitcoin" }, None)
        .await
        .unwrap();
    
    assert!(found_history.is_some());
    let found_history = found_history.unwrap();
    assert_eq!(found_history.prices.len(), 30);
    
    common::cleanup_test_db(&db).await;
}

#[tokio::test]
#[serial]
async fn test_upsert_operation() {
    common::init_test_logger();
    
    let db = common::setup_test_db().await;
    let db_client = DbClient { db: db.clone() };
    
    let collection = db_client.get_tokens_collection();
    
    // First upsert (insert)
    let token = common::mock_data::create_test_token("solana");
    let filter = doc! { "token_id": "solana" };
    
    let update = doc! {
        "$set": {
            "token_id": &token.token_id,
            "current_price": token.current_price,
            "symbol": &token.symbol,
            "name": &token.name,
        }
    };
    
    let options = mongodb::options::UpdateOptions::builder()
        .upsert(true)
        .build();
    
    let result = collection.update_one(filter.clone(), update, options.clone()).await.unwrap();
    assert_eq!(result.upserted_id.is_some(), true);
    
    // Second upsert (update)
    let update2 = doc! {
        "$set": {
            "token_id": "solana",
            "current_price": 150.0,
        }
    };
    
    let result2 = collection.update_one(filter, update2, options).await.unwrap();
    assert_eq!(result2.modified_count, 1);
    
    common::cleanup_test_db(&db).await;
}
