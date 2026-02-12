use mongodb::{Client, Collection, Database};
use once_cell::sync::OnceCell;
use bson::Document;
use std::time::Duration;

static MONGO_CLIENT: OnceCell<Client> = OnceCell::new();

const MONGODB_URI: &str = "mongodb://mustafaterr_db_user:ipBu9XmPvbJsarqD@ac-1i2pgkc-shard-00-00.pyp4ts1.mongodb.net:27017,ac-1i2pgkc-shard-00-01.pyp4ts1.mongodb.net:27017,ac-1i2pgkc-shard-00-02.pyp4ts1.mongodb.net:27017/cluster3?tls=true&replicaSet=atlas-10no6x-shard-0&authSource=admin&retryWrites=true&w=majority";
const DB_NAME: &str = "cluster3";

/// Initialize the MongoDB client. Gracefully handles connection failures.
pub async fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    let mut client_options = mongodb::options::ClientOptions::parse(MONGODB_URI).await?;
    // More generous timeout to allow TLS handshake to complete
    client_options.connect_timeout = Some(Duration::from_secs(10));
    client_options.server_selection_timeout = Some(Duration::from_secs(10));

    let client = Client::with_options(client_options)?;

    // Try to ping, but don't crash if it fails
    match client
        .database("admin")
        .run_command(bson::doc! { "ping": 1 })
        .await
    {
        Ok(_) => log::info!("✅ Connected to MongoDB"),
        Err(e) => log::warn!("⚠️  MongoDB ping failed (will retry on first query): {}", e),
    }

    MONGO_CLIENT
        .set(client)
        .map_err(|_| "MongoDB client already initialized")?;
    Ok(())
}

/// Get a reference to the MongoDB client. Returns None if not initialized.
pub fn try_get_client() -> Option<&'static Client> {
    MONGO_CLIENT.get()
}

/// Get a reference to the MongoDB client.
pub fn get_client() -> &'static Client {
    MONGO_CLIENT
        .get()
        .expect("MongoDB client not initialized. Call init_db() first.")
}

/// Get the default database.
pub fn get_database() -> Database {
    get_client().database(DB_NAME)
}

/// Get the comments collection (as raw Document for flexible queries).
pub fn get_comments_collection() -> Collection<Document> {
    get_database().collection("comments")
}
