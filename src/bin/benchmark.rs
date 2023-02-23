use rusty_sapphire::db_utils::DbUtils;
use rusty_sapphire::listing::Listing;

extern crate reqwest;

use reqwest::header;

#[tokio::main]
async fn main() {
    let items = DbUtils::get_collection_names().await;
    let mut ok = 0;
    for item in items.iter() {
        let client = reqwest::Client::new();

        let response = client.get(Listing::get_url(item, 0, 20))
            .headers(Listing::get_headers())
            .send()
            .await.unwrap();

        if response.status().is_success() {
            ok += 1;
        } else {
            break;
        }
    }

    println!("ok: {}", ok);
}