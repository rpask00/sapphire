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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub _id: ObjectId,
    pub phase_key: String,
    pub market_hash_name: String,
    pub phase: String,
    pub max_buy_price: f64,
}


pub struct DbUtils {
    pub db: Database,
    pub items: Vec<Item>,
}

impl DbUtils {
    pub async fn new(collection_name: &str) -> DbUtils {
        let db = DbUtils::get_db().await;


        let mut db_utils = DbUtils {
            db,
            items: vec![],
        };

        let items = db_utils.get_items(collection_name).await;
        db_utils.items = items;
        db_utils
    }

    async fn get_db() -> Database {
        dotenv().ok();
        let mdb_uri = std::env::var("MDB_URI").expect("MDB_URI environment variable missing");
        let db_name = std::env::var("DB_NAME").expect("DB_NAME environment variable missing");

        let client_options = ClientOptions::parse(&mdb_uri).await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        client.database(&db_name)
    }

    pub async fn get_collection_names() -> Vec<String> {
        let db = DbUtils::get_db().await;
        db.list_collection_names(None).await.unwrap()
    }

    pub async fn get_items(&self, collection_name: &str) -> Vec<Item> {
        let collection = self.db.collection::<Item>(collection_name);
        let cursor = collection.find(None, None).await.unwrap();
        let items: Vec<Result<Item, _>> = cursor.collect::<Vec<_>>().await;


        items.iter().map(|item| item.as_ref().unwrap().clone()).collect()
    }


    pub async fn replace_keys(&mut self, collection_name: &str, new_key: &str, object_id: &ObjectId) {
        for item in self.items.iter_mut(){
            if item._id == *object_id {
                DbUtils::rename_image(&item.phase_key, new_key);

                self.db.collection::<Item>(collection_name).update_one(
                    doc! {"_id": object_id},
                    doc! {"$set": {"phase_key": new_key}},
                    None,
                ).await.unwrap();

                item.phase_key = new_key.to_string();
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
