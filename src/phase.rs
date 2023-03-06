use std::fs::{File};
use std::io::{self, Read};

use serde_derive::Deserialize;
use strum_macros::EnumString;
use crate::db_utils::{DbUtils, Item};


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


impl PHASE {
    pub async fn get_phase_item(phase_key: &str, db_utils: &mut DbUtils) -> Result<Item, io::Error> {

        // println!("Phase key: {}", phase_key);

        for item in db_utils.items.iter() {
            if item.phase_key == phase_key {
                println!("{}", phase_key);
                println!("{}: {}", item.phase, item.phase_key);
                println!("----------------");
                return Ok(item.clone());
            }
        }

        println!("Phase not found");

        let mut found_item: Option<Item> = None;


        for item in db_utils.items.iter() {
            let downloaded_image: Vec<u8> = reqwest::get(PHASE::get_image_url(phase_key))
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap()
                .into_iter()
                .collect();

            let image_from_file: Vec<u8> = File::open(format!("assets/phases/{}.png", item.phase_key))
                .unwrap()
                .bytes()
                .into_iter()
                .map(|x| x.unwrap())
                .collect();

            if Self::images_are_the_same(&downloaded_image, &image_from_file) {
                found_item = Some(item.clone());
                break;
            }
        }

        match found_item {
            Some(item) => {
                db_utils.replace_keys(item.market_hash_name.as_str(), phase_key, &item._id).await;
                Ok(item.clone())
            }
            None => {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Could not find phase",
                ))
            }
        }
    }


    fn images_are_the_same(image1: &[u8], image2: &[u8]) -> bool {
        let mut image2_iter = image2.iter();
        for byte in image1.iter() {
            let byte2 = image2_iter.next().unwrap();

            if *byte != *byte2 {
                println!("not the same");
                return false;
            }
        }
        println!("the same");

        true
    }

    fn get_image_url(phase_key: &str) -> String {
        format!(
            "https://community.cloudflare.steamstatic.com/economy/image/{}/62fx62f",
            phase_key
        )
    }
}

