use std::sync::Arc;
use std::time::Instant;
use reqwest::{Response, StatusCode};
use scraper::Html;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use rusty_sapphire::http_client::HTTPClient;

#[tokio::main]
async fn main() {
    let knifes = vec![
        "★ Karambit | Gamma Doppler (Factory New)",
        "★ StatTrak™ Falchion Knife | Gamma Doppler (Factory New)",
        "★ M9 Bayonet | Gamma Doppler (Minimal Wear)",
        "★ Huntsman Knife | Doppler (Minimal Wear)",
        "★ StatTrak™ Talon Knife | Doppler (Factory New)",
        "★ StatTrak™ M9 Bayonet | Doppler (Factory New)",
        "★ StatTrak™ Stiletto Knife | Doppler (Factory New)",
        "★ Butterfly Knife | Doppler (Factory New)",
        "★ StatTrak™ Karambit | Gamma Doppler (Factory New)",
        "★ StatTrak™ Gut Knife | Doppler (Minimal Wear)",
        "★ StatTrak™ Bowie Knife | Doppler (Factory New)",
        "★ Karambit | Doppler (Factory New)",
        "★ Ursus Knife | Doppler (Factory New)",
        "★ StatTrak™ Gut Knife | Doppler (Factory New)",
    ];


    let mut tasks: Vec<JoinHandle<( i32)>> = Vec::new();
    for knife in knifes {
        tasks.push(tokio::spawn(async move {
            let client = HTTPClient::new().await;
            let (document, total_count) = client.fetch_knife_info(knife, 0, 20).await;


            return total_count;
        }));
    }

    let start = Instant::now();


    let res: i32 = tokio::select! {
        res =  tasks.pop().unwrap() => res.unwrap(),
        res =  tasks.pop().unwrap() => res.unwrap(),
        res =  tasks.pop().unwrap() => res.unwrap(),
        res =  tasks.pop().unwrap() => res.unwrap(),
        res =  tasks.pop().unwrap() => res.unwrap(),
    };

    let duration_secs = start.elapsed().as_secs();

    println!("Pierwszy URL zwrócił status 200 po {duration_secs}s");
}
