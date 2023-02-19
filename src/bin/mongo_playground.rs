// don't forget this!
use mongodb::{
    bson::doc,
    options::FindOptions
};
use mongodb::{
    options::AggregateOptions,
    options::ClientOptions,
    Client
};
use rusty_sapphire::phase::Phase;
use serde::{
    Deserialize,
    Serialize
};
use std::error::Error;
use futures::stream::StreamExt;
use dotenv::dotenv;
use tokio;

#
[derive(Clone, Debug, Deserialize, Serialize)]
struct Item {
    phase_key: String,
    market_hash_name: String,
    phase: String,
    max_buy_price: f64
}



# [tokio::main]
async fn main() {
    dotenv().ok();
    let mdb_uri = std::env::var ("MDB_URI").expect("MDB_URI environment variable missing");
    let db_name = std::env::var ("DB_NAME").expect("DB_NAME environment variable missing");

    // Parse your connection string into an options struct

    let client_options = ClientOptions::parse( & mdb_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database( & db_name);


    for knife_name in db.list_collection_names(None).await.unwrap() {
        let collection = db.collection:: < Item > ( & knife_name);
        let cursor = collection.find(None, None).await.unwrap();



    }




}