use dotenv::dotenv;
use scraper::{Html, Selector};
use serde_json::Value;
use rusty_sapphire::db_utils::DbUtils;
use rusty_sapphire::listing::Listing;
use rusty_sapphire::phase::PHASE;

#[tokio::main]
async fn main() {
    let row_selector = Selector::parse(".market_listing_row").unwrap();

    for knife_name in DbUtils::get_collection_names().await.iter() {
        let mut db_utils = DbUtils::new(knife_name).await;
        loop {
            let document = fetch_knife_info(knife_name).await;


            println!("{}:", knife_name);

            for element in document.select(&row_selector) {
                if let Some(listing) = Listing::new(knife_name, &element) {
                    if let Ok(phase_item) = PHASE::get_phase_item(&listing.phase_key, &mut db_utils).await {
                        println!("max buy price: {}", phase_item.max_buy_price);
                        println!("listing price: {}", listing.price);
                        println!("phase: {:?}", phase_item.phase);
                        println!("should buy: {}\n", phase_item.max_buy_price > listing.price);
                    } else {
                        println!("Error parsing row!");
                    }
                } else {
                    println!("Error parsing row!");
                }
            }

            println!("------------------");
        }
    }
}


async fn fetch_knife_info(knife_name: &String) -> Html {
    let url = Listing::get_url(knife_name);

    loop {
        match client_with_proxy().get(&url).send().await {
            Ok(response) => {
                let status = response.status();
                let text = match response.text().await {
                    Ok(text) => text,
                    Err(_) => {
                        println!("Error parsing response text {}... ", knife_name);
                        continue;
                    }
                };

                let lookup: Value = match serde_json::from_str(&text) {
                    Ok(lookup) => lookup,
                    Err(_) => {
                        println!("Error occurred for {} - code {}", knife_name, status);
                        continue;
                    }
                };

                let lookup = lookup.as_object().unwrap();
                let html = lookup.get("results_html").unwrap().as_str().unwrap();
                return Html::parse_document(html);
            }
            Err(_) => println!("Error while fetching {}... ", knife_name),
        }
    }
}


fn client_with_proxy() -> reqwest::Client {
    dotenv().ok();
    let proxy_url = std::env::var("PROXY_URL").expect("PROXY_URL variable not found");
    reqwest::Client::builder()
        .proxy(reqwest::Proxy::https(proxy_url).unwrap())
        .build()
        .unwrap()
}
