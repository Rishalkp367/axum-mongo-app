use anyhow::{Context, Result};
use mongodb::{Client, Database};

pub async fn init_db(uri: &str, db_name: &str) -> Result<Database> {
    tracing::info!("📡 Connecting to MongoDB at {}", uri);

    let client = Client::with_uri_str(uri)
        .await
        .context("Failed to create Mongo client")?;

    // Force a ping so connection is REAL (not lazy)
    client
        .database("admin")
        .run_command(mongodb::bson::doc! { "ping": 1 }, None)
        .await
        .context("MongoDB ping failed")?;

    tracing::info!(
        "✅ MongoDB connected successfully to database '{}'",
        db_name
    );

    Ok(client.database(db_name))
}
