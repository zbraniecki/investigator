mod assets;
mod identities;
mod markets;
mod prices;
mod wallets;

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
        AddAssetClass,
        RemoveAssetClass,
        ShowAssetClasses,

        AddAsset,
        FetchAssetInfo,
        FetchAssetsInfo,
        ShowAssets,
        RemoveAsset,

        FetchAssetPrices,

        AddIdentity,
        RemoveIdentity,
        ShowIdentities,

        AddWallet,
        RemoveWallet,
        ShowWallets,

        AddPassiveIncome,
        RemovePassiveIncome,
        ClearPassiveIncomes,

        AddMarket,
        RemoveMarket,
        ShowMarkets,
        FetchMarketAssets,

        None,
    }
    let cmd = match args.get(1).map(|s| s.as_str()) {
        Some("add_asset_class") => Command::AddAssetClass,
        Some("remove_asset_class") => Command::RemoveAssetClass,
        Some("show_asset_classes") => Command::ShowAssetClasses,
        Some("add_asset") => Command::AddAsset,
        Some("show_assets") => Command::ShowAssets,
        Some("fetch_asset_info") => Command::FetchAssetInfo,
        Some("fetch_assets_info") => Command::FetchAssetsInfo,
        Some("remove_asset") => Command::RemoveAsset,
        Some("fetch_asset_prices") => Command::FetchAssetPrices,
        Some("add_identity") => Command::AddIdentity,
        Some("remove_identity") => Command::RemoveIdentity,
        Some("show_identities") => Command::ShowIdentities,
        Some("show_wallets") => Command::ShowWallets,
        Some("add_wallet") => Command::AddWallet,
        Some("remove_wallet") => Command::RemoveWallet,
        Some("add_passive_income") => Command::AddPassiveIncome,
        Some("clear_passive_incomes") => Command::ClearPassiveIncomes,
        Some("remove_passive_income") => Command::RemovePassiveIncome,
        Some("add_market") => Command::AddMarket,
        Some("remove_market") => Command::RemoveMarket,
        Some("show_markets") => Command::ShowMarkets,
        Some("fetch_market_assets") => Command::FetchMarketAssets,
        _ => Command::None,
    };
    println!("Command: {:?}", cmd);

    match cmd {
        Command::AddAssetClass => {
            assets::add_asset_class(&args);
        }
        Command::RemoveAssetClass => {
            assets::remove_asset_class(&args);
        }
        Command::ShowAssetClasses => {
            assets::show_asset_classes(&args);
        }
        Command::AddAsset => {
            assets::add_asset(&args);
        }
        Command::FetchAssetInfo => {
            assets::fetch_asset_info(&args).await;
        }
        Command::FetchAssetsInfo => {
            assets::fetch_assets_info(&args).await;
        }
        Command::ShowAssets => {
            assets::show_assets(&args);
        }
        Command::RemoveAsset => {
            assets::remove_asset(&args);
        }
        Command::FetchAssetPrices => {
            prices::fetch_asset_prices(&args).await;
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
        Command::AddWallet => {
            wallets::add_wallet(&args);
        }
        Command::RemoveWallet => {
            wallets::remove_wallet(&args);
        }
        Command::ShowWallets => {
            wallets::show_wallets(&args);
        }
        Command::AddPassiveIncome => {
            wallets::add_passive_income(&args);
        }
        Command::RemovePassiveIncome => {
            wallets::remove_passive_income(&args);
        }
        Command::ClearPassiveIncomes => {
            wallets::clear_passive_incomes(&args);
        }
        Command::AddMarket => {
            markets::add_market(&args);
        }
        Command::RemoveMarket => {
            markets::remove_market(&args);
        }
        Command::ShowMarkets => {
            markets::show_markets(&args);
        }
        Command::FetchMarketAssets => {
            markets::fetch_market_assets(&args).await;
        }
        Command::None => {}
    }
}
