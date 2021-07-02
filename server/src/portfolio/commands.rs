use super::api;
use super::db;
use crate::db::establish_connection;

pub fn get_prefix() -> &'static str {
    "portfolio"
}

pub fn get_list() -> Vec<&'static str> {
    vec![
        "create",
        "read",
        "update",
        "delete",
        "filter",
        "addAsset",
        "removeAsset",
        "clearAssets",
        "syncAssets",
    ]
}

pub async fn handle_command(cmd: &str, args: &[String]) -> bool {
    match cmd {
        "create" => create(args),
        "delete" => delete(args),
        "filter" => filter(args),
        "addAsset" => add_asset(args),
        "syncAssets" => sync_assets(args).await,
        _ => {
            return false;
        }
    }
    true
}

pub fn create(args: &[String]) {
    let connection = establish_connection();
    let slug = args.get(2).unwrap();
    let name = args.get(3).unwrap();
    db::portfolio::create(&connection, slug, name);
}

pub fn delete(args: &[String]) {
    let id: i64 = args.get(2).unwrap().parse().unwrap();
    let connection = establish_connection();
    db::portfolio::delete(&connection, id);
}

pub fn filter(_args: &[String]) {
    let connection = establish_connection();
    let portfolios = db::portfolio::filter(&connection);
    for portfolio in portfolios {
        println!("ID: {}", portfolio.id);
        println!("Slug: {}", portfolio.slug);
        println!("Name: {}", portfolio.name.unwrap_or("-".to_string()));
        let assets = db::portfolio_assets::filter(&connection, portfolio.id);
        println!("{:#?}", assets);
    }
}

pub fn add_asset(args: &[String]) {
    let connection = establish_connection();
    let portfolio: i64 = args.get(2).unwrap().parse().unwrap();
    let asset = args.get(3).unwrap();
    db::portfolio_assets::create(&connection, portfolio, asset);
}

pub async fn sync_assets(args: &[String]) {
    let connection = establish_connection();
    let portfolio_id: i64 = args.get(2).unwrap().parse().unwrap();

    let portfolio = db::portfolio::get(&connection, portfolio_id).unwrap();
    match portfolio.slug.as_str() {
        "top30crypto" => {
            let assets = api::fetch_crypto_info(&portfolio.slug).await.unwrap();
            db::portfolio_assets::clear(&connection, portfolio.id);

            let tag = crate::asset::db::tag::get(&connection, "crypto").unwrap();

            for asset in assets {
                if crate::asset::db::asset::get(&connection, &asset.id).is_none() {
                    crate::asset::db::asset::create(
                        &connection,
                        &asset.id,
                        Some(&asset.symbol),
                        Some(&asset.name),
                    );
                }
                if crate::asset::db::tag::get_for_asset(&connection, &asset.id)
                    .iter()
                    .find(|t| t.tag == tag.id)
                    .is_none()
                {
                    crate::asset::db::tag::add_asset(&connection, &tag.id, &asset.id);
                }
                db::portfolio_assets::create(&connection, portfolio.id, &asset.id);
            }
        }
        "top30stock" | "fidelity" | "ej1" | "ej2" => {
            let assets = api::fetch_stock_info(&portfolio.slug).await.unwrap();
            db::portfolio_assets::clear(&connection, portfolio.id);

            let tag = crate::asset::db::tag::get(&connection, "stock").unwrap();

            for asset in assets {
                if crate::asset::db::asset::get(&connection, &asset.ticker).is_none() {
                    crate::asset::db::asset::create(
                        &connection,
                        &asset.ticker,
                        Some(&asset.ticker),
                        Some(&asset.name),
                    );
                }
                if crate::asset::db::tag::get_for_asset(&connection, &asset.ticker)
                    .iter()
                    .find(|t| t.tag == tag.id)
                    .is_none()
                {
                    crate::asset::db::tag::add_asset(&connection, &tag.id, &asset.ticker);
                }
                db::portfolio_assets::create(&connection, portfolio.id, &asset.ticker);
            }
        }
        _ => {}
    }
}
