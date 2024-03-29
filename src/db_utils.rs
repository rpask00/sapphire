use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use futures::StreamExt;
use mongodb::{Client, Database};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::ClientOptions;
use serde::{
    Deserialize,
    Serialize,
};
use tokio::sync::Mutex;

use crate::config::{PHASES_COLLECTION_NAME, REPLACED_PHASE_KEYS, STEAM_USERS_COLLECTION_NAME};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub _id: ObjectId,
    pub phase_key: String,
    pub market_hash_name: String,
    pub phase: String,
    pub max_buy_price: f64,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SteamUser {
    pub _id: ObjectId,
    pub login: String,
    pub cookie: String,
}


pub struct DbUtils {
    pub db: Arc<Mutex<Database>>,
    pub items: Vec<Item>,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhaseKey {
    phase_key: String,
}


impl DbUtils {
    pub async fn new(market_hash_name: &str, db: Arc<Mutex<Database>>) -> DbUtils {
        let mut db_utils = DbUtils {
            db,
            items: vec![],
        };

        let items = db_utils.get_items(market_hash_name).await;
        db_utils.items = items;
        db_utils
    }

    pub async fn spawn_db_connection() -> Database {
        dotenv().ok();
        let mdb_uri = env::var("MDB_URI").expect("MDB_URI environment variable missing");
        let db_name = env::var("DB_NAME").expect("DB_NAME environment variable missing");

        let client_options = ClientOptions::parse(&mdb_uri).await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        client.database(&db_name)
    }

    pub async fn get_collection_names(db: Arc<Mutex<Database>>) -> Vec<String> {
        let items = db.lock().await.collection::<Item>(PHASES_COLLECTION_NAME).find(None, None).await.unwrap();
        let mut items = items.map(|item| item.unwrap().market_hash_name).collect::<Vec<_>>().await;
        items.dedup();

        items
    }

    pub async fn get_items(&self, market_hash_name: &str) -> Vec<Item> {
        let collection = self.db.lock().await.collection::<Item>(PHASES_COLLECTION_NAME);

        loop {
            if let Ok(cursor) = collection.find(None, None).await {
                let items: Vec<Result<Item, _>> = cursor.collect::<Vec<_>>().await;

                return items.iter().map(|item| item.as_ref().unwrap().clone())
                    .filter(|item| item.market_hash_name == market_hash_name).collect();
            } else {
                println!("Failed fetching items");
            }
        }
    }


    pub async fn replace_keys(&mut self, new_key: &str, object_id: &ObjectId) {
        for item in self.items.iter_mut() {
            if item._id == *object_id {
                DbUtils::rename_image(&item.phase_key, new_key);

                self.db.lock().await.collection::<Item>(PHASES_COLLECTION_NAME).update_one(
                    doc! {"_id": object_id},
                    doc! {"$set": {"phase_key": new_key}},
                    None,
                ).await.unwrap();


                self.db.lock().await.collection::<PhaseKey>(REPLACED_PHASE_KEYS).insert_one(
                    PhaseKey {
                        phase_key: new_key.to_string()
                    },
                    None,
                ).await.unwrap();


                item.phase_key = new_key.to_string();

                break;
            }
        }
    }

    fn rename_image(previous_key: &str, new_phase_key: &str) {
        std::fs::rename(
            format!("assets/phases/{}.png", previous_key),
            format!("assets/phases/{}.png", new_phase_key),
        ).unwrap();
    }
}

impl DbUtils {
    pub async fn get_cookie(db: Arc<Mutex<Database>>) -> String {
        let collection = db.lock().await.collection::<SteamUser>(STEAM_USERS_COLLECTION_NAME);
        let steam_user = env::var("STEAM_USER").expect("STEAM_USER environment variable missing");

        let mut cursor = collection.find(doc! {
            "login": steam_user
        }, None).await.unwrap();

        let decrypted_cookie = cursor.next().await.unwrap().unwrap().cookie;

        let fernet_key = env::var("FERNET_KEY").expect("FERNET_KEY environment variable missing");
        let fernet = fernet::Fernet::new(&fernet_key).unwrap();

        String::from_utf8(fernet.decrypt(&decrypted_cookie).unwrap()).unwrap()
    }
}



