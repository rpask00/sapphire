use mongodb::bson::oid::ObjectId;
use rusty_sapphire::db_utils::DbUtils;


// -9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXH5ApeO4YmlhxYQknCRvCo04DEVlxkKgpovbSsLQJfw-bbeQJD4eOxlY2GlsjwPKvBmm5D19V5i_rEp7P5gVO8v11lZmn6cICSJ1A6Nw2DrgS-l-bphpS578uaySAx73UisCuPlhW_hhhFcKUx0kWzyDYX

#[tokio::main]
async fn main() {
    let mut db_utils = DbUtils::new().await;
    db_utils.collect_all_items().await;
    let items = db_utils.items.get("★ Shadow Daggers | Gamma Doppler (Minimal Wear)").unwrap();


    let item = items.iter().find(|item| item.phase_key == "towjastara").unwrap().clone();


    db_utils.replace_keys(
        item.market_hash_name.as_str(),
        "-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXH5ApeO4YmlhxYQknCRvCo04DEVlxkKgpovbSsLQJfw-bbeQJD4eOxlY2GlsjwPKvBmm5D19V5i_rEp7P5gVO8v11lZmn6cICSJ1A6Nw2DrgS-l-bphpS578uaySAx73UisCuPlhW_hhhFcKUx0kWzyDYX",
        &item._id,
    ).await;

    let items = db_utils.items.get("★ Shadow Daggers | Gamma Doppler (Minimal Wear)").unwrap();
    let item = items.iter().find(|item| item.phase_key == "-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxDZ7I56KU0Zwwo4NUX4oFJZEHLbXH5ApeO4YmlhxYQknCRvCo04DEVlxkKgpovbSsLQJfw-bbeQJD4eOxlY2GlsjwPKvBmm5D19V5i_rEp7P5gVO8v11lZmn6cICSJ1A6Nw2DrgS-l-bphpS578uaySAx73UisCuPlhW_hhhFcKUx0kWzyDYX").unwrap().clone();


    println!("{:?}", item);
}

