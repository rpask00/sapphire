use reqwest::{Client, Proxy};
use scraper::{Html, Selector};
use serde_json::Value;

use rusty_sapphire::config::PROXY_URL;
use rusty_sapphire::listing::Listing;
use rusty_sapphire::phase::{Phase, PHASE};

#[tokio::main]
async fn main() {
    let phase = Phase::new();
    let proxy = Proxy::https(PROXY_URL).unwrap();
    let client = Client::builder().proxy(proxy).build().unwrap();
    let row_selector = Selector::parse(".market_listing_row").unwrap();

    let phase_iter = phase.lookup.as_object().unwrap().iter();
    for (knife_name, phase_lookup) in phase_iter {
        let url = Listing::get_url(&knife_name);
        let response = client.get(url).send().await.unwrap();
        let text = response.text().await.unwrap();

        let lookup: Value = serde_json::from_str(&text).unwrap();
        let lookup = lookup.as_object().unwrap();
        let html = lookup.get("results_html").unwrap().as_str().unwrap();
        let document = Html::parse_document(&html);

        for element in document.select(&row_selector) {
            let listing = Listing::new(&knife_name, &element);
            let phase = Phase::get_phase(&listing.image_hash, &phase_lookup);

            match phase.await {
                Ok(a) => {
                    println!(
                        "{} - {:?}  - {} - {}",
                        knife_name, a, listing.price, listing.buy_order_id
                    );
                }
                Err(_) => {
                    // println!("{} not found for {}", key, knife);
                }
            }
        }
    }
}
