use std::num::ParseFloatError;
use regex::Regex;
use scraper::{ElementRef, Selector};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CONNECTION, REFERER, USER_AGENT};


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

    pub fn get_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "text/javascript, text/html, application/xml, text/xml, */*".parse().unwrap());
        headers.insert(ACCEPT_LANGUAGE, "pl-PL,pl;q=0.5".parse().unwrap());
        headers.insert(CONNECTION, "keep-alive".parse().unwrap());
        headers.insert(REFERER, "https://steamcommunity.com/market/listings/730/AK-47%20%7C%20Phantom%20Disruptor%20%28Field-Tested%29".parse().unwrap());
        headers.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
        headers.insert("Sec-Fetch-Mode", "cors".parse().unwrap());
        headers.insert("Sec-Fetch-Site", "same-origin".parse().unwrap());
        headers.insert("Sec-GPC", "1".parse().unwrap());
        headers.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36".parse().unwrap());
        headers.insert("X-Prototype-Version", "1.7".parse().unwrap());
        headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
        headers.insert("sec-ch-ua", "\"Chromium\";v=\"110\", \"Not A(Brand\";v=\"24\", \"Brave\";v=\"110\"".parse().unwrap());
        headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
        headers.insert("sec-ch-ua-platform", "\"Linux\"".parse().unwrap());
        // headers.insert(COOKIE, "sessionid=2743efb45b5eabbf81ea92d9; timezoneOffset=3600,0; steamCountry=DE%7C710a14a608e46764f27c0d683c83e935")

        headers
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
