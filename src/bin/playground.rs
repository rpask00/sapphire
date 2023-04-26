use std::fs::File;
use std::io::Cursor;
use std::sync::Arc;
use std::time::Duration;
use image::io::Reader as ImageReader;
use sapphire::config::get_image_url;
use sapphire::db_utils::DbUtils;
use fernet;
use tokio::sync::Mutex;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let cookie = Arc::new(Mutex::new(String::new()));

    let cookie_ref = cookie.clone();
    tokio::spawn(async move {
        loop {
            let cookie = DbUtils::get_cookie().await;
            *cookie_ref.lock().await = cookie;
            sleep(Duration::from_secs(3)).await;

            println!("Cookie updated: {}", cookie_ref.lock().await);
        }
    });

    let cookie_ref = cookie.clone();
    tokio::spawn(async move {
        loop {
            let cookie = cookie_ref.lock().await.to_string();
            println!("Cookie read: {}", cookie_ref.lock().await);
            sleep(Duration::from_secs(1)).await;

        }
    });

    sleep(Duration::from_secs(13454353)).await;

}

