use std::fs::File;
use std::io::Cursor;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use image::io::Reader as ImageReader;
use sapphire::config::get_image_url;
use sapphire::db_utils::DbUtils;
use sapphire::utils::printc;

#[tokio::main]
async fn main() {
    let now = SystemTime::now() - Duration::from_secs(67);

    let now2 = SystemTime::now();



    println!("{:?}", now2.duration_since(now));
}


