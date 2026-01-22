use mongodb::{Client, Collection, Database};
use crate::models::{CryptoToken, PriceHistory};

#[derive(Clone)]
pub struct DbClient {
    pub db: Database,
}

impl DbClient {
    pub fn get_tokens_collection(&self) -> Collection<CryptoToken> {
        self.db.collection::<CryptoToken>("tokens")
    }

    pub fn get_history_collection(&self) -> Collection<PriceHistory> {
        self.db.collection::<PriceHistory>("price_history")
    }
}

pub async fn init_db(uri: &str, database_name: &str) -> DbClient {
    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to MongoDB");

    let db = client.database(database_name);

    DbClient { db }
}
