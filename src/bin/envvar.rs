use std::env;

use dotenv::dotenv;

pub mod csmoney;

fn main() {
    dotenv().ok();

    let database_url = env::var("MDB_URL")
        .expect("DATABASE_URL must be set");

    println!("Database URL is: {}", database_url);
}
