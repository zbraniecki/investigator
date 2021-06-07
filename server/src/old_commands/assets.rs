use super::establish_connection;
use crate::api;
use crate::db;
use std::time::Duration;
use tokio::time::delay_for;

pub fn add_asset(args: &[String]) {
    let id = args.get(2).unwrap();
    let class = args.get(3).unwrap();
    let connection = establish_connection();
    db::create_asset(&connection, id, class, None, None);
}

pub fn remove_asset(args: &[String]) {
    let id = args.get(2).unwrap();
    let connection = establish_connection();
    db::remove_asset(&connection, id);
}

pub fn show_assets(_args: &[String]) {
    let connection = establish_connection();
    let assets = db::get_assets(&connection);
    for asset in assets {
        println!("ID: {}", asset.id);
        println!("Class: {}", asset.class);
        println!("Symbol: {}", asset.symbol.unwrap_or("-".to_string()));
        println!("Name: {}\n", asset.name.unwrap_or("-".to_string()));
        let prices = db::fetch_prices(&connection, &asset.id, "usd");
        println!("{:#?}", prices);
        println!("----------");
    }
}

pub async fn fetch_asset_info(args: &[String]) {
    let id = args.get(2).unwrap();
    let class = args.get(3).unwrap();
    let asset_info = api::fetch_asset_info(&id).await.unwrap();
    let connection = establish_connection();
    db::set_asset_info(&connection, id, class, &asset_info);
}

pub async fn fetch_assets_info(_args: &[String]) {
    let connection = establish_connection();
    let assets = db::get_assets(&connection);
    for asset in assets {
        print!("Fetching info for {}", &asset.id);
        let asset_info = api::fetch_asset_info(&asset.id).await.unwrap();
        db::set_asset_info(&connection, &asset.id, &asset.class, &asset_info);
        println!("   DONE!");
        delay_for(Duration::from_millis(1000)).await;
    }
}

pub fn add_asset_class(args: &[String]) {
    let id = args.get(2).unwrap();
    let name = args.get(3).unwrap();
    let connection = establish_connection();
    db::create_asset_class(&connection, id, name);
}

pub fn remove_asset_class(args: &[String]) {
    let id = args.get(2).unwrap();
    let connection = establish_connection();
    db::remove_asset_class(&connection, id);
}

pub fn show_asset_classes(_args: &[String]) {
    let connection = establish_connection();
    let classes = db::get_asset_classes(&connection);
    for class in classes {
        println!("ID: {}", class.id);
        println!("Name: {}", class.name);
        println!("----------");
    }
}
