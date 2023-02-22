use tokio::time::sleep;
use std::time::{Duration, Instant};
use dotenv::dotenv;
use futures::stream::iter;
use scraper::{Html, Selector};
use serde_json::Value;
use rusty_sapphire::db_utils::DbUtils;
use rusty_sapphire::listing::Listing;
use rusty_sapphire::pager::Pager;
use rusty_sapphire::phase::PHASE;


#[tokio::main]
async fn main() {
    let names: Vec<String> = DbUtils::get_collection_names().await.iter().skip(0).take(96).cloned().collect();


    for knife_name in names {
        // let row_selector = Selector::parse(".market_listing_row").unwrap();
        let mut pager = Pager::new();

        tokio::spawn(async move {
            // let mut db_utils = DbUtils::new(&knife_name).await;
            let mut start = Instant::now();
            loop {
                while pager.next().unwrap() {
                    match async {
                        start = Instant::now();
                        let (_document, total_count) = fetch_knife_info(&knife_name, pager.start, pager.count).await;
                        pager.set_total_count(total_count);

                        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
                    }
                        .await
                    {
                        Ok(()) => {
                            let duration = start.elapsed();
                            println!("sekund: {} - {}", duration.as_secs(), &knife_name);
                        }
                        Err(er) => println!("{}", er),
                    }
                }
            }
        });


        // for element in document.select(&row_selector) {
        //     if let Some(listing) = Listing::new(knife_name, &element) {
        //         if let Ok(phase_item) = PHASE::get_phase_item(&listing.phase_key, &mut db_utils).await {
        //             println!("max buy price: {}", phase_item.max_buy_price);
        //             println!("listing price: {}", listing.price);
        //             println!("phase: {:?}", phase_item.phase);
        //             println!("should buy: {}\n", phase_item.max_buy_price > listing.price);
        //         } else {
        //             println!("Error parsing row!");
        //         }
        //     } else {
        //         println!("Error parsing row!");
        //     }
        // }
    }
    sleep(Duration::from_secs(60 * 60 * 24 * 31 * 12 * 100)).await;
}


async fn fetch_knife_info(knife_name: &String, start: i32, count: i32) -> (Html, i32) {
    let url = Listing::get_url(knife_name, start, count);

    loop {
        match client_with_proxy().get(&url).send().await {
            Ok(response) => {
                let status = response.status();
                let text = match response.text().await {
                    Ok(text) => text,
                    Err(_) => {
                        // println!("Error parsing response text {}... ", knife_name);
                        continue;
                    }
                };

                let lookup: Value = match serde_json::from_str(&text) {
                    Ok(lookup) => lookup,
                    Err(_) => {
                        // println!("Error occurred for {} - code {}", knife_name, status);
                        continue;
                    }
                };

                let lookup = lookup.as_object().unwrap();
                let html = lookup.get("results_html").unwrap().as_str().unwrap();
                let total_count = lookup.get("total_count").unwrap().as_u64().unwrap();
                return (Html::parse_document(html), total_count as i32);
            }
            Err(_) => {
                // println!("Error while fetching {}... ", knife_name)
                continue;
            }
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

    // loop {
    //     if let Ok(response) = client.get("https://api.ipify.org").send().await {
    //         let my_ip = response.text().await.unwrap();
    //         // println!("Ip address: {}", my_ip);
    //
    //         return client;
    //     }
    // }
}
