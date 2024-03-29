// don't forget this!
use mongodb::{bson::doc};
use mongodb::{options::ClientOptions};
use serde::{Deserialize, Serialize};
use std::error::Error;
use futures::stream::StreamExt;
use dotenv::dotenv;

use std::fs::File;
use std::io::prelude::*;

use image::io::Reader as ImageReader;
use sapphire::config::get_image_url;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Item {
    phase_key: String,
    market_hash_name: String,
    phase: String,
    max_buy_price: f64,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let mdb_uri = std::env::var("MDB_URI").expect("MDB_URI environment variable missing");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME environment variable missing");

    // Parse your connection string into an options struct
    let client_options = ClientOptions::parse(&mdb_uri).await.unwrap();

    let client = mongodb::Client::with_options(client_options).unwrap();

    let db = client.database(&db_name);
    let http_client = reqwest::Client::new();


    for knife_name in db.list_collection_names(None).await.unwrap() {
        let collection = db.collection::<Item>(&knife_name);

        let mut cursor = collection.find(None, None).await.unwrap();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(item) => {
                    let response = http_client.get(get_image_url(&item.phase_key)).send().await?;


                    let buffer = response.bytes().await?.to_vec();
                    let image = ImageReader::new(std::io::Cursor::new(buffer)).with_guessed_format()?.decode()?;
                    let mut file = File::create(format!("assets/phases/{}.png", item.phase_key))?;
                    image.write_to(&mut file, image::ImageOutputFormat::Png)?;
                }
                Err(e) => println!("Error {:?}", e),
            }
        }
    }

    Ok(())
}
