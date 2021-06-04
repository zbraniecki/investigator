use super::establish_connection;
use crate::api;
use crate::db;

pub fn add_market(args: &[String]) {
    let id = args.get(2).unwrap();
    let name = args.get(3).unwrap();
    let connection = establish_connection();
    db::create_market(&connection, id, name);
}

pub fn remove_market(args: &[String]) {
    let id = args.get(2).unwrap();
    let connection = establish_connection();
    db::remove_market(&connection, id);
}

pub fn show_markets(_args: &[String]) {
    let connection = establish_connection();
    let markets = db::get_markets(&connection);
    for market in markets {
        println!("ID: {}", market.id);
        println!("Name: {}", market.name);
        println!("----------");
        let market_assets = db::get_market_assets(&connection, &market.id);
        println!("{:#?}", market_assets);
    }
}

pub async fn fetch_market_assets(args: &[String]) {
    let id = args.get(2).unwrap();
    let market_assets = api::fetch_market_assets(&id).await.unwrap();
    let connection = establish_connection();
    db::set_market_assets(&connection, id, &market_assets);
}
