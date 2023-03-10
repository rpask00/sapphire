use dotenv::dotenv;
use reqwest::header::{ACCEPT, HeaderMap, REFERER};
use serde_json::Value;

use crate::config::{Currency, dummy_headers};
use crate::listing;
use crate::listing::Listings;

pub struct HTTPClient {
    proxy_url: String,
}


impl HTTPClient {
    pub async fn new() -> HTTPClient {
        dotenv().ok();
        let proxy_url = std::env::var("PROXY_URL").expect("PROXY_URL variable not found");

        HTTPClient {
            proxy_url,
        }
    }

    fn client_with_proxy(&self) -> reqwest::Client {
        reqwest::Client::builder()
            .proxy(reqwest::Proxy::https(&self.proxy_url).unwrap())
            .build()
            .unwrap()

        // loop {
        //     if let Ok(response) = client.get("https://api.ipify.org").send().await {
        //         let my_ip = response.text().await.unwrap();
        //         // println!("Ip address: {}", my_ip);
        //
        //         return client;
        //     }
        // }
    }


    pub async fn fetch_knife_info(&self, knife_name: &String, start: i32, count: i32) -> Result<Listings, listing::Error> {
        let url = HTTPClient::get_url(knife_name, start, count);

        loop {
            match self.client_with_proxy()
                .get(&url)
                .headers(HTTPClient::get_headers(knife_name))
                .send().await {
                Ok(response) => {
                    let _status = response.status();
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
    fn get_headers(knife_name: &String) -> HeaderMap {
        let mut headers = dummy_headers();
        headers.insert(ACCEPT, "text/javascript, text/html, application/xml, text/xml, */*".parse().unwrap());
        headers.insert(REFERER, format!("https://steamcommunity.com/market/listings/730/{knife_name}").parse().unwrap());
        headers.insert("X-Prototype-Version", "1.7".parse().unwrap());
        headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
        // headers.insert(COOKIE, "sessionid=2743efb45b5eabbf81ea92d9; timezoneOffset=3600,0; steamCountry=DE%7C710a14a608e46764f27c0d683c83e935")

        headers
    }

    fn get_url(name: &String, start: i32, count: i32) -> String {
        format!("https://steamcommunity.com/market/listings/730/{name}/render/?query=&start={start}&count={count}&country=PL&language=english&currency={}", i32::from(Currency::PLN))
    }
}
