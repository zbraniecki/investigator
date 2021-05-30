#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod db;
pub mod api;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", 
                                   database_url))
}

fn add_coin(args: &[String]) {
    let connection = establish_connection();
    db::create_coin(&connection, "btc", "btc", "Bitcoin");
}

async fn fetch_coin_info(args: &[String]) {
    let id = args.get(1).unwrap();
    let coin_info = api::fetch_coin_info(&id).await.unwrap();
    // let connection = establish_connection();
    // db::create_coin(&connection, "btc", "btc", "Bitcoin");
}

#[actix_web::main]
async fn main() {
    #[derive(Debug)]
    enum Command {
        AddCoin,
        FetchCoinInfo,
        // RemoveCoin,
        None,
    }

    let args: Vec<String> = env::args().collect();
    let cmd = match args.get(1).map(|s| s.as_str()) {
        Some("add_coin") => Command::AddCoin,
        Some("fetch_coin_info") => Command::FetchCoinInfo,
        _ => Command::None,
    };
    println!("Command: {:?}", cmd);

    match cmd {
        Command::AddCoin => {
            add_coin(&args);
        },
        Command::FetchCoinInfo => {
            fetch_coin_info(&args);
        },
        Command::None => {}
    }
    // db::create_coin(&connection, "usd", "usd", "US Dollar");
    // db::set_coin_price(&connection, "btc", "usd", 35000.0);
    // let results = coins
    //     .limit(5)
    //     .load::<Coin>(&connection)
    //     .expect("Error loading posts");

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
