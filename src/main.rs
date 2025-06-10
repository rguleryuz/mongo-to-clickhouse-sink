use std::env;

use futures::StreamExt;
use mongodb::{bson::Document, options::ClientOptions, Client, Collection};
use serde::Serialize;

#[derive(Serialize)]
struct RawDoc {
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI not set");
    let mongo_db = env::var("MONGODB_DB").expect("MONGODB_DB not set");
    let mongo_coll = env::var("MONGODB_COLL").expect("MONGODB_COLL not set");
    let ch_url = env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL not set");
    let ch_db = env::var("CLICKHOUSE_DATABASE").expect("CLICKHOUSE_DATABASE not set");
    let ch_table = env::var("CLICKHOUSE_TABLE").expect("CLICKHOUSE_TABLE not set");

    let mut options = ClientOptions::parse(mongo_uri).await?;
    let client = Client::with_options(options)?;
    let collection: Collection<Document> = client.database(&mongo_db).collection(&mongo_coll);

    let ch = clickhouse::Client::default()
        .with_url(ch_url)
        .with_database(ch_db);

    let mut stream = collection.watch(None, None).await?;
    println!("watching for changes...");

    while let Some(event) = stream.next().await {
        match event {
            Ok(change) => {
                if let Some(doc) = change.full_document {
                    let data = serde_json::to_string(&doc)?;
                    let row = RawDoc { data };
                    if let Err(e) = ch.insert(&ch_table).one(row).await {
                        eprintln!("failed to insert into clickhouse: {}", e);
                    }
                }
            }
            Err(e) => eprintln!("error reading change stream: {}", e),
        }
    }

    Ok(())
}
