use std::fs::File;
use std::io::Cursor;

use image::io::Reader as ImageReader;

use sapphire::config::get_image_url;
use sapphire::db_utils::DbUtils;

#[tokio::main]
async fn main() {
    let db = DbUtils::new("★ Karambit | Doppler (Factory New)").await;

    for (i,item) in db.items.iter().enumerate(){
        println!("{i}, {}", item.phase);
    }
}


async fn save_to_file(key: &str, location: &str) {
    let response = reqwest::get(get_image_url(key)).await.unwrap();
    let buffer = response.bytes().await.unwrap().to_vec();
    let image = ImageReader::new(Cursor::new(buffer)).with_guessed_format().unwrap().decode().unwrap();
    let mut file = File::create(format!("{}/{}.png", location, key)).unwrap();
    image.write_to(&mut file, image::ImageOutputFormat::Png).unwrap();
}
