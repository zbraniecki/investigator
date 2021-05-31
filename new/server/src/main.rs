#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod api;
pub mod db;
pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn add_coin(args: &[String]) {
    let id = args.get(2).unwrap();
    let connection = establish_connection();
    db::create_coin(&connection, id, None, None);
}

fn remove_coin(args: &[String]) {
    let id = args.get(2).unwrap();
    let connection = establish_connection();
    db::remove_coin(&connection, id);
}

fn show_coins(_args: &[String]) {
    let connection = establish_connection();
    let coins = db::get_coins(&connection);
    for coin in coins {
        println!("ID: {}", coin.id);
        println!("----------");
        println!("Symbol: {}", coin.symbol.unwrap_or("-".to_string()));
        println!("Name: {}\n\n", coin.name.unwrap_or("-".to_string()));
    }
}

async fn fetch_coin_info(args: &[String]) {
    let id = args.get(2).unwrap();
    let coin_info = api::fetch_coin_info(&id).await.unwrap();
    let connection = establish_connection();
    db::set_coin_info(&connection, id, &coin_info);
}

#[actix_web::main]
async fn main() {
    #[derive(Debug)]
    enum Command {
        AddCoin,
        FetchCoinInfo,
        ShowCoins,
        RemoveCoin,
        None,
    }

    let args: Vec<String> = env::args().collect();
    let cmd = match args.get(1).map(|s| s.as_str()) {
        Some("add_coin") => Command::AddCoin,
        Some("show_coins") => Command::ShowCoins,
        Some("fetch_coin_info") => Command::FetchCoinInfo,
        Some("remove_coin") => Command::RemoveCoin,
        _ => Command::None,
    };
    println!("Command: {:?}", cmd);

    match cmd {
        Command::AddCoin => {
            add_coin(&args);
        }
        Command::FetchCoinInfo => {
            fetch_coin_info(&args).await;
        }
        Command::ShowCoins => {
            show_coins(&args);
        }
        Command::RemoveCoin => {
            remove_coin(&args);
        }
        Command::None => {}
    }
    // println!("Displaying {} coins", results.len());
    // for coin in results {
    //     println!("{}", coin.id);
    //     println!("----------");
    //     println!("{}", coin.symbol);
    //     println!("{}\n\n", coin.name);

    //     let price = db::get_current_price(&connection, &coin.id, "usd");
    //     if let Some(price) = price {
    //         println!("USD: {}", price.value);
    //     } else {
    //         println!("USD: ???");
    //     }
    // }
}
