use std::sync::Arc;
use tokio::sync::Mutex;
use sapphire::db_utils::DbUtils;
use sapphire::http_client::HTTPClient;
use sapphire::listing::{Asset, Listing};


#[tokio::main]
async fn main() {
    let asset = Asset {
        currency: 0,
        id: "".to_string(),
        classid: "".to_string(),
        instanceid: "".to_string(),
        original_amount: "".to_string(),
        unowned_id: "".to_string(),
        unowned_contextid: "".to_string(),
        icon_url: "".to_string(),
        market_hash_name: "AWP | Pit Viper (Field-Tested)".to_string(),
    };

    let listing = Listing {
        converted_price: 281,
        converted_publisher_fee: 28,
        converted_steam_fee: 14,
        total_price: 0.0,
        asset,
        listingid: "4300417711580759122".to_string(),
    };

    let cookie = DbUtils::get_cookie(
        Arc::new(Mutex::new(DbUtils::spawn_db_connection().await))
    ).await;
    HTTPClient::buy_knife(&listing, &cookie).await;

}

