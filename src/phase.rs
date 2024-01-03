use std::fs::File;
use std::io::{self, Read};

use image::io::Reader as ImageReader;
use serde_derive::Deserialize;
use strum_macros::EnumString;

use crate::config::get_image_url;
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
        for item in db_utils.items.iter() {
            if item.phase_key == phase_key {
                return Ok(item.clone());
            }
        }

        let mut found_item: Option<Item> = None;
        PHASE::save_to_file(phase_key, "assets/phases/temporary").await;

        for item in db_utils.items.iter() {
            let image_from_file: Vec<u8> = PHASE::image_from_file(item.phase_key.as_str(), "assets/phases");
            let downloaded_image: Vec<u8> = PHASE::image_from_file(phase_key, "assets/phases/temporary");

            if Self::images_are_the_same(&downloaded_image, &image_from_file) {
                found_item = Some(item.clone());
                break;
            }
        }

        match found_item {
            Some(item) => {
                db_utils.replace_keys(phase_key, &item._id).await;
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
                return false;
            }
        }
        true
    }



    async fn save_to_file(key: &str, location: &str) {
        let response = reqwest::get(get_image_url(key)).await.unwrap();
        let buffer = response.bytes().await.unwrap().to_vec();
        let image = ImageReader::new(io::Cursor::new(buffer)).with_guessed_format().unwrap().decode().unwrap();
        let mut file = File::create(format!("{}/{}.png", location, key)).unwrap();
        image.write_to(&mut file, image::ImageOutputFormat::Png).unwrap();
    }

    fn image_from_file(key: &str, location: &str) -> Vec<u8> {
        File::open(format!("{}/{}.png", location, key))
            .unwrap()
            .bytes()
            .map(|x| x.unwrap())
            .collect()
    }
}

