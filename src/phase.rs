use std::fs::{File};
use std::io::{self, ErrorKind, Read};
use std::str::FromStr;

use reqwest::{Client};
use serde_derive::Deserialize;
use serde_json::Value;
use strum_macros::EnumString;

#[derive(Debug, Deserialize, EnumString)]
pub enum PHASE {
    Phase1,
    Phase2,
    Phase3,
    Phase4,
    Sapphire,
    Ruby,
    BlackPearl,
    Emerald,
}

pub async fn get_phase(key: &str, lookup: &Value) -> Result<PHASE, io::Error> {
    let client = Client::new();

    match lookup.get(key) {
        Some(phase) => {
            return Ok(PHASE::from_str(phase.as_str().unwrap()).unwrap());
        }
        None => {
            for key2 in lookup.as_object().unwrap().keys() {
                let url = format!(
                    "https://community.cloudflare.steamstatic.com/economy/image/{}/62fx62f",
                    key
                );
                let downloaded_image: Vec<u8> = client
                    .get(url)
                    .send()
                    .await
                    .unwrap()
                    .bytes()
                    .await
                    .unwrap()
                    .into_iter()
                    .collect();

                let image_from_file: Vec<u8> = File::open(format!("assets/phases/{}.png", key2))
                    .unwrap()
                    .bytes()
                    .into_iter()
                    .map(|x| x.unwrap())
                    .collect();

                if images_are_the_same(&downloaded_image, &image_from_file) {
                    let phase =
                        PHASE::from_str(lookup.get(key2).unwrap().as_str().unwrap()).unwrap();
                    return Ok(phase);
                }
            }
            Err(io::Error::new(
                ErrorKind::NotFound,
                "phase does not match the key",
            ))
        }
    }
}


pub fn images_are_the_same(image1: &[u8], image2: &[u8]) -> bool {
    let mut image2_iter = image2.iter();
    for byte in image1.iter() {
        let byte2 = image2_iter.next().unwrap();

        if *byte != *byte2 {
            return false;
        }
    }

    true
}
