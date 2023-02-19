//use reqwest::{Client, Proxy};
//use rusty_sapphire::config::PROXY_URL;
//use serde_json::Value;
//
//#[tokio::main]
//async fn main() {
//    //    let proxy = Proxy::all(PROXY_URL).unwrap();
//    //    let client = reqwest::Client::builder().proxy(proxy).build().unwrap();
//    let client = Client::builder()
//        .proxy(Proxy::all("http://rp.261000.gmail.com:rzzrhk@gate2.proxyfuel.com:2000").unwrap())
//        .build()
//        .unwrap();
//
//    for i in 1..10 {
//        //        let response = client.get("https://httpbin.org/ip").send().await.unwrap();
//        //        let text = response.text().await.unwrap();
//        //
//        //        println!("{}", text);
//        //    }
//
//        let resp = client.get("https://httpbin.org/ip").send().await.unwrap();
//
//        println!("status: {}", resp.text().await.unwrap());
//    }
//}

use reqwest::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    send_get_request_with_proxy().await;
}

async fn send_get_request_with_proxy() -> Result<(), reqwest::Error> {
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .build()?;

    let headers = reqwest::header::HeaderMap::new();

    for i in 0..10 {
        match client.get("http://checkip.amazonaws.com")
            .proxy(reqwest::Proxy::http("http://asdf.gmail.com:asdfasdf@gate2.proxyfuel.com:2000").unwrap())
            .send()
            .await {
            Ok(resp) => println!("{}", resp.text().await?),
            Err(e) => println!("{}", e),
        }
    }

    Ok(())
}