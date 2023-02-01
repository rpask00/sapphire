use reqwest::{Client, Proxy};
use scraper::{Html, Selector};
use serde_json::Value;

use rusty_sapphire::config::proxy_url;
use rusty_sapphire::listing::Listing;
use rusty_sapphire::phase;
use rusty_sapphire::phase::{PHASE, Phase};

#[tokio::main]
async fn main() {
    let phase = Phase::new();

    let obj = phase.lookup.as_object().unwrap().iter();

    for (knife, phase_lookup) in obj {
        let proxy = Proxy::https(proxy_url).unwrap();

        let client = Client::builder().proxy(proxy).build().unwrap();

        let url = Listing::get_url(&knife);
        let response = client.get(url).send().await.unwrap();
        let text = response.text().await.unwrap();

        let lookup: Value = serde_json::from_str(&text).unwrap();
        let lookup = lookup.as_object().unwrap();
        let html = lookup.get("results_html").unwrap().as_str().unwrap();
        let document = Html::parse_document(&html);
        let row_selector = Selector::parse(".market_listing_row").unwrap();


        for element in document.select(&row_selector) {
            let listing = Listing::new(&knife, &element, &phase_lookup);
            let phase = phase::get_phase(&listing.image_hash, &phase_lookup);


            match phase.await {
                Ok(a) => {
                    println!("{} - {:?}  - {}", knife, a, listing.price);
                }
                Err(_) => {
                    // println!("{} not found for {}", key, knife);
                }
            }
        }
    }
}


