use dotenv::dotenv;
use scraper::Html;
use serde_json::Value;
use reqwest::header::{HeaderMap, ACCEPT, ACCEPT_LANGUAGE, CONNECTION, REFERER, USER_AGENT};

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

    pub async fn fetch_knife_info(&self, knife_name: &String, start: i32, count: i32) -> (Html, i32) {
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

                    let lookup = lookup.as_object().unwrap();
                    let html = lookup.get("results_html").unwrap().as_str().unwrap();
                    let total_count = lookup.get("total_count").unwrap().as_u64().unwrap();
                    return (Html::parse_document(html), total_count as i32);
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
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "text/javascript, text/html, application/xml, text/xml, */*".parse().unwrap());
        headers.insert(ACCEPT_LANGUAGE, "pl-PL,pl;q=0.5".parse().unwrap());
        headers.insert(CONNECTION, "keep-alive".parse().unwrap());
        headers.insert(REFERER, format!("https://steamcommunity.com/market/listings/730/{knife_name}").parse().unwrap());
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

    fn get_url(name: &String, start: i32, count: i32) -> String {
        format!("https://steamcommunity.com/market/listings/730/{name}/render/?query=&start={start}&count={count}&country=PL&language=english&currency=6")
    }
}