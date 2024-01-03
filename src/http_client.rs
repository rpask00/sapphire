use std::collections::HashMap;
use std::time::SystemTime;

use dotenv::dotenv;
use reqwest::{header, StatusCode};
use reqwest::header::{ACCEPT, HeaderMap, REFERER};
use serde_json::Value;
use tokio::time::sleep;

use crate::config::{buylisting_params, Currency, dummy_headers};
use crate::listing;
use crate::listing::{Listing, Listings};

pub struct HTTPClient {
    proxy_url: String,
    proxy_availability: HashMap<String, SystemTime>,
}


impl HTTPClient {
    pub async fn new(proxies: Vec<String>) -> HTTPClient {
        dotenv().ok();
        let proxy_url = std::env::var("PROXY_URL").expect("PROXY_URL variable not found");

        let mut proxy_availability = HashMap::new();
        for proxy in proxies {
            proxy_availability.insert(proxy, SystemTime::now() - std::time::Duration::from_secs(600));
        }

        HTTPClient {
            proxy_url,
            proxy_availability
        }
    }

    async fn client_with_proxy(&self) -> (String, reqwest::Client) {


        let mut banned = 0;
        let count = self.proxy_availability.len();

        loop {
            let random_index = rand::random::<usize>() % count;
            let (proxy_url, time) = self.proxy_availability.iter().nth(random_index).unwrap();

            if time.elapsed().unwrap().as_secs() > 60 {
                let client = reqwest::Client::builder()
                    .proxy(reqwest::Proxy::https(proxy_url.to_owned()).unwrap())
                    .build()
                    .unwrap();

                // println!("{banned} / {count} banned");
                return (proxy_url.to_string(), client);
            } else {
                banned += 1;
            }
            sleep(std::time::Duration::from_secs(10)).await;
        }
    }



    pub async fn fetch_knife_info(&mut self, knife_name: &String, start: i32, count: i32) -> Result<Listings, listing::Error> {
        let url = HTTPClient::fetch_url(knife_name, start, count);

        loop {
            let (proxy_url, client) = self.client_with_proxy().await;

            match client
                .get(&url)
                .headers(HTTPClient::fetch_headers(knife_name))
                .send().await {
                Ok(response) => {
                    let status = response.status();

                    if status == StatusCode::TOO_MANY_REQUESTS {
                        self.proxy_availability.insert(proxy_url, SystemTime::now());
                        continue;
                    }

                    let text = match response.text().await {
                        Ok(text) => text,
                        Err(_) => {
                            // println!("Error parsing response text {}... ", knife_name);
                            continue;
                        }
                    };

                    let lookup: Value = match serde_json::from_str(&text) {
                        Ok(lookup) => lookup,
                        Err(_) => {
                            // println!("Error occurred for {} - code {}", knife_name, status);
                            continue;
                        }
                    };


                    return Listings::from_value(&lookup);
                }
                Err(_) => {
                    // println!("Error while fetching {}... ", knife_name)
                    continue;
                }
            }
        }
    }
}

impl HTTPClient {
    fn fetch_headers(knife_name: &String) -> HeaderMap {
        let mut headers = dummy_headers();
        headers.insert(ACCEPT, "text/javascript, text/html, application/xml, text/xml, */*".parse().unwrap());
        headers.insert(REFERER, format!("https://steamcommunity.com/market/listings/730/{knife_name}").parse().unwrap());
        headers.insert("X-Prototype-Version", "1.7".parse().unwrap());
        headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
        // headers.insert(COOKIE, "sessionid=2743efb45b5eabbf81ea92d9; timezoneOffset=3600,0; steamCountry=DE%7C710a14a608e46764f27c0d683c83e935")

        headers
    }

    fn buying_headers(knife_name: &String, cookie: &str) -> HeaderMap {
        let mut headers = dummy_headers();
        headers.insert(ACCEPT, "*/*".parse().unwrap());
        headers.insert(REFERER, format!("https://steamcommunity.com/market/listings/730/{}", knife_name).parse().unwrap());
        headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
        headers.insert("Origin", "https://steamcommunity.com".parse().unwrap());
        headers.insert(header::COOKIE, cookie.parse().unwrap());

        headers
    }

    fn get_sessionid(cookie: &str) -> String {
        cookie.split("; ").find(|&x| x.starts_with("sessionid=")).unwrap().split('=').nth(1).unwrap().to_string()
    }

    fn fetch_url(name: &String, start: i32, count: i32) -> String {
        format!("https://steamcommunity.com/market/listings/730/{name}/render/?query=&start={start}&count={count}&country=PL&language=english&currency={}", i32::from(Currency::PLN))
    }
}


impl HTTPClient {
    pub async fn buy_knife(listing: &Listing, cookie: &str) {
        let sessionid = HTTPClient::get_sessionid(cookie);
        let headers = HTTPClient::buying_headers(&listing.asset.market_hash_name, cookie);


        let subtotal = listing.converted_price;
        let fee = listing.converted_publisher_fee + listing.converted_steam_fee;
        let total = subtotal + fee;

        let params = buylisting_params(&sessionid, subtotal, fee, total);

        let client = reqwest::Client::new();
        let res = client.post(format!("https://steamcommunity.com/market/buylisting/{}", listing.listingid))
            .headers(headers)
            .body(params.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<String>>().join("&"))
            .send().await.unwrap();


        println!("{}", res.status());
    }
}
