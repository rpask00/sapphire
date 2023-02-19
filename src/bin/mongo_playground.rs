use scraper::Selector;
use serde::{
    Serialize,
};
use tokio;

use rusty_sapphire::db_utils;


#[tokio::main]
async fn main() {
    let row_selector = Selector::parse(".market_listing_row").unwrap();


    for knife_name in db_utils.collection_names {}
}


