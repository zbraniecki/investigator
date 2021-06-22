use super::super::api;
use super::super::db;
use crate::db::establish_connection;

pub fn get_prefix() -> &'static str {
    "asset"
}

pub fn get_list() -> Vec<&'static str> {
    vec!["create", "read", "update", "delete", "filter", "fetchInfo"]
}

pub async fn handle_command(cmd: &str, args: &[String]) -> bool {
    match cmd {
        "create" => create(args),
        "delete" => delete(args),
        "filter" => filter(args),
        "fetchInfo" => fetch_info(args).await,
        _ => {
            return false;
        }
    }
    true
}

pub fn create(args: &[String]) {
    let connection = establish_connection();
    let id = args.get(2).unwrap();
    db::asset::create(&connection, id, None, None);
}

pub fn delete(args: &[String]) {
    let connection = establish_connection();
    let id = args.get(2).unwrap();
    db::asset::delete(&connection, id);
}

pub fn filter(_args: &[String]) {
    let connection = establish_connection();
    let assets = db::asset::filter(&connection, None);
    for asset in assets {
        println!("ID: {}", asset.id);
        println!("Symbol: {}", asset.symbol.unwrap_or("-".to_string()));
        println!("Name: {}\n", asset.name.unwrap_or("-".to_string()));
        let prices = crate::price::db::fetch(&connection, &asset.id, "usd");
        println!("{:#?}", prices);
        println!("----------");
    }
}

pub async fn fetch_info(args: &[String]) {
    let id = args.get(2).unwrap();
    let asset_info = api::crypto::fetch_info(&id).await.unwrap();
    let connection = establish_connection();
    db::asset::set_info(&connection, id, &asset_info);
}
