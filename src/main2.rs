use dotenv::dotenv;
use reqwest::{Client, Error, Proxy, Response};
use scraper::{Html, Selector};
use serde_json::Value;

use rusty_sapphire::listing::Listing;
use rusty_sapphire::phase::{Phase, PHASE};

#[tokio::main]
async fn main() {
    let phase = Phase::new();
    let row_selector = Selector::parse(".market_listing_row").unwrap();

    let phase_iter = phase.lookup.as_object().unwrap().iter();
    for (knife_name, phase_lookup) in phase_iter {
        let response = fetch_knife_info(&knife_name).await;
        let status = response.status();
        let text = response.text().await.unwrap();




        let lookup: Value = match serde_json::from_str(&text) {
            Ok(lookup) => lookup,
            Err(_) => {
                println!("Error occurred for {} - code {}", knife_name, status);
                continue;
            }
        };
        let lookup = lookup.as_object().unwrap();
        let html = lookup.get("results_html").unwrap().as_str().unwrap();
        let document = Html::parse_document(&html);

        for element in document.select(&row_selector) {
            let listing = Listing::new(&knife_name, &element);
            let phase = Phase::get_phase(&listing.image_hash, &phase_lookup);

            match phase.await {
                Ok(phase) => {
                    match phase { 
                        PHASE::Sapphire | PHASE::Ruby | PHASE::BlackPearl | PHASE::Emerald => {
                            println!(
                                "{} - {:?}  - {} - {}",
                                knife_name, phase, listing.price, listing.buy_order_id
                            );
                        }
                        _ => {
                            println!(".");
                        }
                    }
                }
                Err(_) => {
                    // println!("{} not found for {}", key, knife);
                }
            }
        }
    }
}


async fn fetch_knife_info(knife_name: &String) -> Response {
    let url = Listing::get_url(&knife_name);

    loop {
        match client_with_proxy().get(&url).send().await {
            Ok(response) => {
                return response;
            },
            Err(_) => {
                println!("Error occurred for {}", knife_name);
            }
        }
    }
}


fn client_with_proxy() -> Client {
    dotenv().ok();
    let proxy_url =  std::env::var("PROXY_URL").expect("PROXY_URL variable not found");
    Client::builder()
        .proxy(Proxy::https(&proxy_url).unwrap())
        .build()
        .unwrap()
}
