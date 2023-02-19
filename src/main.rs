use dotenv::dotenv;
use scraper::{Html, Selector};
use serde_json::Value;
use rusty_sapphire::db_utils::DbUtils;
use rusty_sapphire::listing::Listing;
use rusty_sapphire::phase::PHASE;

#[tokio::main]
async fn main() {
    let row_selector = Selector::parse(".market_listing_row").unwrap();
    let mut db_utils = DbUtils::new().await;
    db_utils.collect_all_items().await;

    let collection_names = db_utils.collection_names.clone();

    for knife_name in collection_names.iter() {
        let document = fetch_knife_info(knife_name).await;


        for element in document.select(&row_selector) {
            let listing = Listing::new(knife_name, &element);
            let phase = PHASE::get_phase(knife_name, &listing.image_hash, &mut db_utils).await.unwrap();

            println!("{} - {} - {:?}", listing.name, listing.price, phase)
        }

        println!("------------------");
    }
}


async fn fetch_knife_info(knife_name: &String) -> Html {
    let url = Listing::get_url(knife_name);

    loop {
        match client_with_proxy().get(&url).send().await {
            Ok(response) => {
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
