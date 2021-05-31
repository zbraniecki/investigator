use std::time::Duration;
use tokio::time::delay_for;
use super::establish_connection;
use crate::db;
use crate::api;

pub fn add_coin(args: &[String]) {
    let id = args.get(2).unwrap();
    let connection = establish_connection();
    db::create_coin(&connection, id, None, None);
}

pub fn remove_coin(args: &[String]) {
    let id = args.get(2).unwrap();
    let connection = establish_connection();
    db::remove_coin(&connection, id);
}

pub fn show_coins(_args: &[String]) {
    let connection = establish_connection();
    let coins = db::get_coins(&connection);
    for coin in coins {
        println!("ID: {}", coin.id);
        println!("----------");
        println!("Symbol: {}", coin.symbol.unwrap_or("-".to_string()));
        println!("Name: {}\n\n", coin.name.unwrap_or("-".to_string()));
    }
}

pub async fn fetch_coin_info(args: &[String]) {
    let id = args.get(2).unwrap();
    let coin_info = api::fetch_coin_info(&id).await.unwrap();
    let connection = establish_connection();
    db::set_coin_info(&connection, id, &coin_info);
}

pub async fn fetch_coins_info(_args: &[String]) {
    let connection = establish_connection();
    let coins = db::get_coins(&connection);
    for coin in coins {
        print!("Fetching info for {}", &coin.id);
        let coin_info = api::fetch_coin_info(&coin.id).await.unwrap();
        db::set_coin_info(&connection, &coin.id, &coin_info);
        println!("   DONE!");
        delay_for(Duration::from_millis(1000)).await;
    }
}
