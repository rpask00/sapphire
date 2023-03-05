use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::config::TAX_MULTIPLIER;
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
    pub price: f64,
    pub listingid: String,
    pub asset: Asset,
}

impl Listing {
    pub fn new(price: f64, listingid: String, asset: Asset) -> Listing {
        Listing {
            price,
            listingid,
            asset,
        }
    }
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
            let assetid = lookup.pointer("/asset/id").unwrap().as_str().unwrap();
            let asset = assets.iter().find(|asset| asset.id == assetid).unwrap();
            let mut price = 0.0;


            match lookup.get("converted_price") {
                Some(value) => {
                    price = value.as_f64().unwrap() * TAX_MULTIPLIER;
                }
                None => {
                    printc("Item sold", red);
                    return None;
                }
            }

            let listingid = lookup.get("listingid").unwrap().as_str().unwrap();

            Some(Listing {
                price,
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
