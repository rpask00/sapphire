use scraper::{ElementRef, Selector};
use serde_json::Value;
use crate::phase::PHASE;

pub struct Listing {
    pub name: String,
    // phase: PHASE,
    pub image_hash: String,
    pub price: f64,
    // listing_id: String,
}

impl Listing {
    pub fn get_url(name: &String) -> String {
        format!("https://steamcommunity.com/market/listings/730/{}/render/?query=&start=0&count=100&country=PL&language=english&currency=6", name)
    }

    pub fn new(name: &String, element: &ElementRef, lookup: &Value) -> Listing {
        // TODO: use '?' and return result
        let img_selector = Selector::parse(".market_listing_item_img_container img").unwrap();
        let price_selector = Selector::parse(".market_listing_price_with_fee").unwrap();


        let image = element.select(&img_selector).nth(0).unwrap();
        let mut price = element.select(&price_selector).nth(0).unwrap().inner_html();

        for c in vec![" ", "zł"] {
            price = price.replace(c, "");
        };
        price = price.replace(",", ".");

        let price = price.trim().to_string().parse::<f64>().unwrap();


        let image_url = image.value().attr("src").unwrap();
        let image_hash = image_url
            .split_once("/image/")
            .unwrap()
            .1
            .split_once("/")
            .unwrap()
            .0;

        Listing {
            name: name.to_string(),
            price,
            image_hash: image_hash.to_string(),
        }
    }
}
