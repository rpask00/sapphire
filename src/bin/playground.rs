use std::fs::File;
use std::io::Cursor;
use image::io::Reader as ImageReader;
use rusty_sapphire::config::get_image_url;

#[tokio::main]
async fn main() {
   save_to_file("-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXH5ApeO4YmlhxYQknCRvCo04DEVlxkKgpotLu8JAllx8zJfAJG48ymmIWZqOf8MqjUx1Rd4cJ5ntbN9J7yjRrmrxZrZGH6JoaSdgZrZwvU-lPvk-i-1pW66svMnHtnuyAj7HmLzUC_n1gSOSy4kjfm", "assets/phases").await;
}


async fn save_to_file(key: &str, location: &str) {
    let response = reqwest::get(get_image_url(key)).await.unwrap();
    let buffer = response.bytes().await.unwrap().to_vec();
    let image = ImageReader::new(Cursor::new(buffer)).with_guessed_format().unwrap().decode().unwrap();
    let mut file = File::create(format!("{}/{}.png", location, key)).unwrap();
    image.write_to(&mut file, image::ImageOutputFormat::Png).unwrap();
}
