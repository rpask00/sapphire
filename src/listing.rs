use regex::Regex;
use scraper::{ElementRef, Selector};

pub struct Listing {
    pub name: String,
    // phase: PHASE,
    pub image_hash: String,
    pub price: f64,
    pub buy_order_id: String,
}

impl Listing {
    pub fn get_url(name: &String) -> String {
        format!("https://steamcommunity.com/market/listings/730/{}/render/?query=&start=0&count=100&country=PL&language=english&currency=6", name)
    }

    pub fn new(name: &String, element: &ElementRef) -> Listing {
        Listing {
            image_hash: Self::get_image_hash(element),
            name: name.to_string(),
            price: Self::get_price(element),
            buy_order_id: Self::get_buy_order_id(element),
        }
    }

    fn get_price(element: &ElementRef) -> f64 {
        let price_selector = Selector::parse(".market_listing_price_with_fee").unwrap();
        let mut price = element.select(&price_selector).next().unwrap().inner_html();

        for c in &[" ", "zł"] {
            price = price.replace(c, "");
        }
        price = price.replace(',', ".");

        price.trim().to_string().parse::<f64>().unwrap_or_else(|_| panic!("Error parsing price: {}", price))
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
