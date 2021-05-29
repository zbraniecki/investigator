#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod db;

use crate::models::Coin;
use crate::schema::coins::dsl::*;
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

fn main() {
    println!("Hello, world!");
    let connection = establish_connection();

    db::create_coin(&connection, "btc", "btc", "Bitcoin");
    db::create_coin(&connection, "usd", "usd", "US Dollar");
    db::set_coin_price(&connection, "btc", "usd", 35000.0);
    let results = coins
        .limit(5)
        .load::<Coin>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} coins", results.len());
    for coin in results {
        println!("{}", coin.id);
        println!("----------");
        println!("{}", coin.symbol);
        println!("{}\n\n", coin.name);

        let price = db::get_current_price(&connection, &coin.id, "usd");
        if let Some(price) = price {
            println!("USD: {}", price.value);
        } else {
            println!("USD: ???");
        }
    }
}
