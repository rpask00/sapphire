use tokio::time::sleep;
use std::time::{Duration};
use rusty_sapphire::db_utils::DbUtils;
use rusty_sapphire::http_client::HTTPClient;
use rusty_sapphire::listing::Error;
use rusty_sapphire::pager::Pager;
use rusty_sapphire::phase::PHASE;
use rusty_sapphire::utils::{printc, red, yellow};


#[tokio::main]
async fn main() {
    let names: Vec<String> = DbUtils::get_collection_names().await.iter().skip(0).take(96).cloned().collect();
    // let names: Vec<String> = vec!["★ Karambit | Doppler (Factory New)".to_string()];


    for knife_name in names {
        let mut pager = Pager::new();
        let http_client = HTTPClient::new().await;
        let mut db_utils = DbUtils::new(&knife_name).await;

        tokio::spawn(async move {
            loop {
                loop {
                    let listings = http_client.fetch_knife_info(&knife_name, pager.start, pager.count).await;
                    // println!("{}: {} - {} - total: {}", Blue.paint(&knife_name), pager.start, pager.count, Red.paint(total_count.to_string()));

                    match listings {
                        Ok(listings) => {
                            pager.set_total_count(listings.total_count);
                            for listing in listings.listings {
                                if let Ok(phase_item) = PHASE::get_phase_item(&listing.asset.icon_url, &mut db_utils).await {
                                    // println!("{} --- max:{} --- price:{} ", phase_item.phase, phase_item.max_buy_price, listing.price);
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

                    if !pager.next().unwrap() {
                        break;
                    }
                }
            }
        });
    }
    sleep(Duration::from_secs(60 * 60 * 24 * 31 * 12 * 100)).await;
}
