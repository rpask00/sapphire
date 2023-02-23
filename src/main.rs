use tokio::time::sleep;
use std::time::{Duration, Instant};
use rusty_sapphire::db_utils::DbUtils;
use rusty_sapphire::http_client::HTTPClient;
use rusty_sapphire::pager::Pager;


#[tokio::main]
async fn main() {
    let names: Vec<String> = DbUtils::get_collection_names().await.iter().skip(0).take(96).cloned().collect();


    for knife_name in names {
        // let row_selector = Selector::parse(".market_listing_row").unwrap();
        let mut pager = Pager::new();

        tokio::spawn(async move {
            let http_client = HTTPClient::new().await;
            // let mut db_utils = DbUtils::new(&knife_name).await;
            let mut start = Instant::now();
            loop {
                while pager.next().unwrap() {
                    match async {
                        start = Instant::now();
                        let (_document, total_count) = http_client.fetch_knife_info(&knife_name, pager.start, pager.count).await;
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

