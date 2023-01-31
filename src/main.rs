use std::fs::{read_to_string};
use reqwest::{Client, Proxy};
use scraper::{Html, Selector};
use serde_json::Value;

use rusty_sapphire::phase;

#[tokio::main]
async fn main() {
    let phases_content = read_to_string("assets/doppler_phases.json").unwrap();
    let phases = serde_json::from_str::<Value>(&phases_content).unwrap();

    let obj = phases.as_object().unwrap().iter();

    for (knife, phase_lookup) in obj {
        let proxy =
            Proxy::https("http://rp.261000.gmail.com:rzzrhk@gate2.proxyfuel.com:2000").unwrap();

        let client = Client::builder().proxy(proxy).build().unwrap();

        let url = format!("https://steamcommunity.com/market/listings/730/{}/render/?query=&start=0&count=100&country=PL&language=english&currency=6", knife);
        let response = client.get(url).send().await.unwrap();
        let text = response.text().await.unwrap();

        let lookup: Value = serde_json::from_str(&text).unwrap();
        let lookup = lookup.as_object().unwrap();
        let html = lookup.get("results_html").unwrap().as_str().unwrap();
        let document = Html::parse_document(&html);
        let row_selector = Selector::parse(".market_listing_row").unwrap();
        let img_selector = Selector::parse(".market_listing_item_img_container img").unwrap();
        let price_selector = Selector::parse(".market_listing_price_with_fee").unwrap();

        for element in document.select(&row_selector) {
            let image = element.select(&img_selector).nth(0).unwrap();
            let price = element.select(&price_selector).nth(0).unwrap().inner_html();
            let price = price.replace(".", "").replace("zł", "").replace(" ", "").replace(",", ".");
            let price = price.trim();
            let price = price.parse::<f64>().unwrap();
            let image_url = image.value().attr("src").unwrap();
            let phase_key = image_url
                .split_once("/image/")
                .unwrap()
                .1
                .split_once("/")
                .unwrap()
                .0;
            let phase = phase::get_phase(phase_key, &phase_lookup);

            match phase.await {
                Ok(a) => {
                    println!("{} - {:?}  - {}", knife, a, price);
                }
                Err(_) => {
                    // println!("{} not found for {}", key, knife);
                }
            }
        }
    }
}


