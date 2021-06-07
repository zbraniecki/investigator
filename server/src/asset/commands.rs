use super::db;
use crate::db::establish_connection;

pub fn get_prefix() -> &'static str {
    "asset"
}

pub fn get_list() -> Vec<&'static str> {
    vec!["create", "read", "update", "delete", "filter"]
}

pub fn handle_command(cmd: &str, args: &[String]) {
    match cmd {
        "create" => create(args),
        "delete" => delete(args),
        "filter" => filter(args),
        _ => {}
    }
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
    let assets = db::asset::filter(&connection);
    for asset in assets {
        println!("ID: {}", asset.id);
        println!("Symbol: {}", asset.symbol.unwrap_or("-".to_string()));
        println!("Name: {}\n", asset.name.unwrap_or("-".to_string()));
        // let prices = db::fetch_prices(&connection, &asset.id, "usd");
        // println!("{:#?}", prices);
        // println!("----------");
    }
}
