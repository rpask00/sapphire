use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::utils::{printc, red};


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No listings found")]
    NoListings,
    #[error("Listings found but not received")]
    ListingsFoundButNotReceived,
    #[error("Total count not present")]
    TotalCountNotPresent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub currency: u32,
    pub id: String,
    pub classid: String,
    pub instanceid: String,
    pub original_amount: String,
    pub unowned_id: String,
    pub unowned_contextid: String,
    pub icon_url: String,
    pub icon_url_large: String,
    pub market_hash_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    pub total_price: f64,
    pub converted_price: u64,
    pub converted_publisher_fee: u64,
    pub converted_steam_fee: u64,
    pub listingid: String,
    pub asset: Asset,
}


pub struct Listings {
    pub listings: Vec<Listing>,
    pub total_count: i32,
}

impl Listings {
    pub fn from_value(lookup: &Value) -> Result<Listings, Error> {
        let total_count = lookup.get("total_count")
            .ok_or(Error::TotalCountNotPresent)?.as_u64().unwrap();

        let assets: Vec<Asset> = lookup.pointer("/assets/730/2").ok_or({
            match total_count {
                0 => Error::NoListings,
                _ => Error::ListingsFoundButNotReceived,
            }
        })?.as_object().unwrap().values().map(|lookup| {
            serde_json::from_value::<Asset>(lookup.to_owned()).unwrap()
        }).collect();


        let listings: Vec<Listing> = lookup.get("listinginfo")
            .ok_or(Error::NoListings)?.as_object().unwrap().values().filter_map(|lookup| {
            let assetid = lookup.pointer("/asset/id")?.as_str()?;
            let asset = assets.iter().find(|asset| asset.id == assetid)?;
            let converted_price = lookup.get("converted_price")?.as_u64()?;
            let converted_publisher_fee = lookup.get("converted_publisher_fee")?.as_u64()?;
            let converted_steam_fee = lookup.get("converted_steam_fee")?.as_u64()?;

            let total_price = (converted_price + converted_publisher_fee + converted_steam_fee) as f64 / 100.0;


            let listingid = lookup.get("listingid")?.as_str()?;

            Some(Listing {
                total_price,
                converted_price,
                converted_publisher_fee,
                converted_steam_fee,
                listingid: listingid.to_string(),
                asset: asset.to_owned(),
            })
        }).collect();


        Ok(Listings {
            listings,
            total_count: total_count as i32,
        })
    }
}
