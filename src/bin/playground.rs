use std::fs::File;
use std::io::Read;
use mongodb::bson::oid::ObjectId;
use rusty_sapphire::db_utils::DbUtils;


#[tokio::main]
async fn main() {
    let currkey = "-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXH5ApeO4YmlhxYQknCRvCo04DEVlxkKgpotLu8JAllx8zJfAJG48ymmIWZqOf8MqjUx1Rd4cJ5nqeT8Ymi3wzt-UNrZ2mmItWRcgRvM16BqVK4l7jq0J-4vZ3IwHQ16HUq-z-DyALmsiWI";
    let newkey = "-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXH5ApeO4YmlhxYQknCRvCo04DEVlxkKgpotLu8JAllx8zJfAJG48ymmIWZqOf8MqjUx1Rd4cJ5ntbN9J7yjRrmrxZrZGH6JoaSdgZrZwvU-lPvk-i-1pW66svMnHtnuyAj7HmLzUC_n1gSOSy4kjfm";

    let currimg = get_img(currkey);
    let newimg = donwloadImage(newkey).await;

    let images_are_the_same = images_are_the_same(&currimg, &newimg);

    println!("Images are the same: {}", images_are_the_same);
}


fn get_img(key: &str) -> Vec<u8> {
    File::open(format!("/home/rfl/dev/doppler/rusty_sapphire/assets/phases/test/{}.png", key))
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
        .into_iter()
        .collect()
}

fn images_are_the_same(image1: &[u8], image2: &[u8]) -> bool {
    let mut image2_iter = image2.iter();
    for byte in image1.iter() {
        let byte2 = image2_iter.next().unwrap();

        if *byte != *byte2 {
            return false;
        }
    }

    true
}


fn get_image_url(phase_key: &str) -> String {
    format!(
        "https://community.cloudflare.steamstatic.com/economy/image/{}/62fx62f",
        phase_key
    )
}

