use tokio::time::sleep;
use std::time::{Duration, Instant};
use futures::StreamExt;
use rusty_sapphire::db_utils::DbUtils;
use rusty_sapphire::http_client::HTTPClient;
use rusty_sapphire::pager::Pager;


#[tokio::main]
async fn main() {
    let names: Vec<String> = DbUtils::get_collection_names().await.iter().skip(0).take(90).cloned().collect();
    let mut tasks = Vec::new();

    for knife_name in names {
        // let row_selector = Selector::parse(".market_listing_row").unwrap();
        let mut pager = Pager::new();


        tasks.push(tokio::spawn(async move {
            let http_client = HTTPClient::new().await;
            let iterations_count = 9;
            let mut sum = 0;
            let mut start = Instant::now();
            // let mut db_utils = DbUtils::new(&knife_name).await;

            for _ in 0..iterations_count {
                while pager.next().unwrap() {
                    match async {
                        start = Instant::now();
                        let (total_count, _) = http_client.fetch_knife_info_concurrent(&knife_name, pager.start, pager.count).await;
                        pager.set_total_count(total_count);
                        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
                    }
                        .await
                    {
                        Ok(()) => {
                            let duration = start.elapsed();
                            sum += duration.as_secs();
                            // println!("sekund: {} - {}", duration.as_secs(), &knife_name);
                        }
                        Err(er) => println!("{}", er),
                    }
                }
            }
            let avg = (sum as f64) / (iterations_count as f64);
            println!("average: {avg}s   - {}", &knife_name);
        }));


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

    for t in tasks {
        t.await.unwrap();
    }
    // sleep(Duration::from_secs(60 * 60 * 24 * 31 * 12 * 100)).await;
}

