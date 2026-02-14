use mongodb::{Client, Database};

pub async fn init_db(uri: &str, db_name: &str) -> Database {
    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to MongoDB");

    println!("✅ Connected to MongoDB");

    client.database(db_name)
}
