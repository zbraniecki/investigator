mod coins;
mod identities;
mod prices;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn handle_command(args: &[String]) {
    #[derive(Debug)]
    enum Command {
        AddCoin,
        FetchCoinInfo,
        FetchCoinsInfo,
        ShowCoins,
        RemoveCoin,

        FetchCoinPrices,

        AddIdentity,
        RemoveIdentity,
        ShowIdentities,
        None,
    }
    let cmd = match args.get(1).map(|s| s.as_str()) {
        Some("add_coin") => Command::AddCoin,
        Some("show_coins") => Command::ShowCoins,
        Some("fetch_coin_info") => Command::FetchCoinInfo,
        Some("fetch_coins_info") => Command::FetchCoinsInfo,
        Some("remove_coin") => Command::RemoveCoin,
        Some("fetch_coin_prices") => Command::FetchCoinPrices,
        Some("add_identity") => Command::AddIdentity,
        Some("remove_identity") => Command::RemoveIdentity,
        Some("show_identities") => Command::ShowIdentities,
        _ => Command::None,
    };
    println!("Command: {:?}", cmd);

    match cmd {
        Command::AddCoin => {
            coins::add_coin(&args);
        }
        Command::FetchCoinInfo => {
            coins::fetch_coin_info(&args).await;
        }
        Command::FetchCoinsInfo => {
            coins::fetch_coins_info(&args).await;
        }
        Command::ShowCoins => {
            coins::show_coins(&args);
        }
        Command::RemoveCoin => {
            coins::remove_coin(&args);
        }
        Command::FetchCoinPrices => {
            prices::fetch_coin_prices(&args).await;
        }
        Command::AddIdentity => {
            identities::add_identity(&args);
        }
        Command::RemoveIdentity => {
            identities::remove_identity(&args);
        }
        Command::ShowIdentities => {
            identities::show_identities(&args);
        }
        Command::None => {}
    }
}
