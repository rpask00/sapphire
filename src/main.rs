use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::sleep;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;
use sapphire::config::proxy_list;
use sapphire::db_utils::DbUtils;
use sapphire::http_client::HTTPClient;
use sapphire::listing::Error;
use sapphire::pager::Pager;
use sapphire::phase::PHASE;
use sapphire::utils::{green, printc, red, yellow};
// use std::sync::{Arc, Mutex};


#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(DbUtils::spawn_db_connection().await));
    let cookie = Arc::new(Mutex::new(String::new()));

    let proxy_availability_hashmap = Arc::new(Mutex::new(HashMap::new()));

    for ip in proxy_list() {
        proxy_availability_hashmap.lock().await.insert(ip, SystemTime::now() - Duration::from_secs(1000));
    }


    let names: Vec<String> = DbUtils::get_collection_names(db.clone()).await;

    // let names: Vec<String> = vec!["★ Stiletto | Doppler (Factory New)".to_string()];

    let db_ref = db.clone();
    let cookie_ref = cookie.clone();
    tokio::spawn(async move {
        loop {
            let cookie = DbUtils::get_cookie(db_ref.clone()).await;
            *cookie_ref.lock().await = cookie;
            sleep(Duration::from_secs(20 * 60)).await;   // Fetch cookies every 20 min
        }
    });


    for knife_name in names {
        let mut pager = Pager::new();
        let mut db_utils = DbUtils::new(&knife_name, db.clone()).await;
        let proxy_availability_hashmap_ref = proxy_availability_hashmap.clone();
        let http_client = HTTPClient::new(proxy_availability_hashmap_ref.clone()).await;
        let cookie_ref = cookie.clone();
        tokio::spawn(async move {
            loop {
                let now = std::time::Instant::now();
                loop {
                    let listings = http_client.fetch_knife_info(&knife_name, pager.start, pager.count).await;

                    match listings {
                        Ok(listings) => {
                            pager.set_total_count(listings.total_count);
                            for listing in listings.listings {
                                if let Ok(phase_item) = PHASE::get_phase_item(&listing.asset.icon_url, &mut db_utils).await {
                                    // println!("{} --- max:{} --- price:{} ", phase_item.phase, phase_item.max_buy_price, listing.price);

                                    if phase_item.max_buy_price >= listing.total_price {
                                        // println!("{} {}  ---- {} + {} + {} = {}", knife_name, listing.listingid, listing.converted_price, listing.converted_publisher_fee, listing.converted_steam_fee, listing.total_price);
                                        let cookie = cookie_ref.lock().await.to_string();
                                        HTTPClient::buy_knife(&listing, &cookie).await;
                                    }
                                } else {
                                    printc("Phase item not found", red);
                                }
                            }
                        }
                        Err(err) => {
                            match err {
                                Error::NoListings => pager.set_total_count(0),
                                _ => printc(format!("{}", err), red)
                            }
                        }
                    }
                    print_benchmark_info(now.elapsed().as_secs(), &knife_name);

                    if !pager.next().unwrap() {
                        break;
                    }
                }
            }
        });
    }
    sleep(Duration::from_secs(60 * 60 * 24 * 31 * 12 * 100)).await;
}

fn print_benchmark_info(duration: u64, knife_name: &String) {
    let mut knife_name = knife_name.clone();
    let len = if knife_name.contains("StatTrak") { 70 } else { 72 };

    knife_name.push_str(" ".repeat(len - knife_name.len()).as_str());

    let message = format!("{}{}s", &knife_name, duration);

    let color = match duration {
        0..=8 => green,
        9..=15 => yellow,
        _ => red
    };

    printc(message, color);
}
