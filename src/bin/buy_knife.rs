use rusty_sapphire::http_client::HTTPClient;
use rusty_sapphire::listing::{Asset, Listing};

fn main() {
    let asset = Asset {
        currency: 0,
        id: "".to_string(),
        classid: "".to_string(),
        instanceid: "".to_string(),
        original_amount: "".to_string(),
        unowned_id: "".to_string(),
        unowned_contextid: "".to_string(),
        icon_url: "".to_string(),
        icon_url_large: "".to_string(),
        market_hash_name: "AWP | Pit Viper (Field-Tested)".to_string(),
    };

    let listing = Listing {
        converted_price: 238,
        converted_publisher_fee: 23,
        converted_steam_fee: 11,
        total_price: 0.0,
        asset,
        listingid: "4272265776381939780".to_string(),
    };

    HTTPClient::buy_knife(&listing);
}

