use std::fs::File;
use std::io::Read;
use mongodb::bson::oid::ObjectId;
use rusty_sapphire::db_utils::DbUtils;
use image::io::Reader as ImageReader;


#[tokio::main]
async fn main() {
    let currkey = "-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXH5ApeO4YmlhxYQknCRvCo04DEVlxkKgpotLu8JAllx8zJfAJG48ymmIWZqOf8MqjUx1Rd4cJ5nqeT8Ymi3wzt-UNrZ2mmItWRcgRvM16BqVK4l7jq0J-4vZ3IwHQ16HUq-z-DyALmsiWI";
    let newkey = "-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXH5ApeO4YmlhxYQknCRvCo04DEVlxkKgpotLu8JAllx8zJfAJG48ymmIWZqOf8MqjUx1Rd4cJ5ntbN9J7yjRrmrxZrZGH6JoaSdgZrZwvU-lPvk-i-1pW66svMnHtnuyAj7HmLzUC_n1gSOSy4kjfm";

    save_to_file(currkey, "assets/phases/test").await;
    save_to_file(newkey, "assets/phases/test").await;


    let currimg = get_img(currkey);
    // let newimg = get_img(newkey);
    let newimg = donwloadImage(newkey).await;

    let images_are_the_same = images_are_the_same(&currimg, &newimg);

    println!("Images are the same: {}", images_are_the_same);
}


fn get_img(key: &str) -> Vec<u8> {
    File::open(format!("assets/phases/test/{}.png", key))
        .unwrap()
        .bytes()
        .into_iter()
        .map(|x| x.unwrap())
        .collect()
}

async fn donwloadImage(key: &str) -> Vec<u8> {
    reqwest::get(get_image_url(key))
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .to_vec()
}

fn images_are_the_same(image1: &[u8], image2: &[u8]) -> bool {
    let mut count = 0;
    let mut image2_iter = image2.iter();
    for byte in image1.iter() {
        let byte2 = image2_iter.next().unwrap();

        if *byte != *byte2 {
            count += 1;
            println!("{} != {}", byte, byte2);
        }
    }
    println!("Count: {}", count);
    count == 0
}


fn get_image_url(phase_key: &str) -> String {
    format!("https://community.cloudflare.steamstatic.com/economy/image/{}/62fx62f", phase_key)
}


async fn save_to_file(key: &str, location: &str) {
    let response = reqwest::get(get_image_url(key)).await.unwrap();
    let buffer = response.bytes().await.unwrap().to_vec();
    let image = ImageReader::new(std::io::Cursor::new(buffer)).with_guessed_format().unwrap().decode().unwrap();
    let mut file = File::create(format!("{}/{}.png", location, key)).unwrap();
    image.write_to(&mut file, image::ImageOutputFormat::Png).unwrap();
}
