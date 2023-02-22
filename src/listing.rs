use std::num::ParseFloatError;
use regex::Regex;
use scraper::{ElementRef, Selector};

pub struct Listing {
    // phase: PHASE,
    pub phase_key: String,
    pub price: f64,
    pub buy_order_id: String,
}

impl Listing {
    pub fn get_url(name: &String, start: i32, count: i32) -> String {
        format!("https://steamcommunity.com/market/listings/730/{name}/render/?query=&start={start}&count={count}&country=PL&language=english&currency=6")
    }

    pub fn new(element: &ElementRef) -> Option<Listing> {
        if let Ok(price) = Self::get_price(element) {
            Some(Listing {
                phase_key: Self::get_image_hash(element),
                price,
                buy_order_id: Self::get_buy_order_id(element),
            })
        } else {
            None
        }
    }

    fn get_price(element: &ElementRef) -> Result<f64, ParseFloatError> {
        let price_selector = Selector::parse(".market_listing_price_with_fee").unwrap();
        let mut price = element.select(&price_selector).next().unwrap().inner_html();

        for c in &[" ", "zł"] {
            price = price.replace(c, "");
        }
        price = price.replace(',', ".");

        return price.trim().to_string().parse::<f64>();
    }

    fn get_image_hash(element: &ElementRef) -> String {
        let img_selector = Selector::parse(".market_listing_item_img_container img").unwrap();

        let image = element.select(&img_selector).next().unwrap();
        let image_url = image.value().attr("src").unwrap();
        let image_hash = image_url
            .split_once("/image/")
            .unwrap()
            .1
            .split_once('/')
            .unwrap()
            .0;

        image_hash.to_string()
    }

    fn get_buy_order_id(element: &ElementRef) -> String {
        let buy_btn_selector = Selector::parse(".market_listing_buy_button a").unwrap();
        let re = Regex::new(&format!(r"{}(.*?){}", "'2', '", "'")).unwrap();

        let href = element
            .select(&buy_btn_selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();

        let buy_order_id = re
            .captures(href)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string();
        buy_order_id
    }
}
