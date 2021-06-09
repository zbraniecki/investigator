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
    ]
}

pub fn handle_command(cmd: &str, args: &[String]) -> bool {
    match cmd {
        "create" => create(args),
        "delete" => delete(args),
        "filter" => filter(args),
        "addAsset" => add_asset(args),
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
    let id: i32 = args.get(2).unwrap().parse().unwrap();
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
    let portfolio: i32 = args.get(2).unwrap().parse().unwrap();
    let asset = args.get(3).unwrap();
    db::portfolio_assets::create(&connection, portfolio, asset);
}
