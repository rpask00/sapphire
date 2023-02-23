use mongodb::bson::oid::ObjectId;
use rusty_sapphire::db_utils::DbUtils;


fn main() {
    let x: u32 = 45;
    let y: u32 = 67;

    println!("{}", y - x);
}

