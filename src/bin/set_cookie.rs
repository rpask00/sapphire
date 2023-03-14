use std::env;
use std::thread::sleep;

use mongodb::{bson::doc};
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;
use dotenv::dotenv;
use dotenv::var;
use dotenv::set_var;
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Setting {
    key: String,
    value: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mdb_uri = env::var("MDB_URI").expect("MDB_URI must be set");
    let fernet_key = env::var("FERNET_KEY").expect("FERNET_KEY must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");

    let client = mongodb::Client::with_uri_str(&mdb_uri).await.expect("Failed to initialize client.");
    let db = client.database(&db_name);
    let settings = db.collection::<Setting>("settings");

    let fernet = fernet::Fernet::new(fernet_key.as_str()).unwrap();

    loop {
        let mut cursor = settings.find(
            doc! {"key": "cookie"},
            None,
        ).await.unwrap();

        let cookie: Setting = cursor.next().await.unwrap().unwrap();
        let decrypted = String::from_utf8(fernet.decrypt(cookie.value.as_str()).unwrap()).unwrap();

        println!("Cookie set: {}", decrypted);
        env::set_var("COOKIE", decrypted);


        sleep(std::time::Duration::from_secs(1));
    }
}

