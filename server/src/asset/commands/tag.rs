use super::super::db;
use crate::db::establish_connection;

pub fn get_prefix() -> &'static str {
    "asset_tag"
}

pub fn get_list() -> Vec<&'static str> {
    vec!["create", "read", "update", "delete", "filter", "addAsset"]
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
    return true;
}

pub fn create(args: &[String]) {
    let connection = establish_connection();
    let id = args.get(2).unwrap();
    db::tag::create(&connection, id);
}

pub fn delete(args: &[String]) {
    let connection = establish_connection();
    let id = args.get(2).unwrap();
    db::tag::delete(&connection, id);
}

pub fn filter(_args: &[String]) {
    let connection = establish_connection();
    let tags = db::tag::filter(&connection);
    for tag in tags {
        println!("ID: {}", tag.id);
        let assets = db::tag::get_assets(&connection, &tag.id);
        println!("{:#?}", assets);
    }
}

pub fn add_asset(args: &[String]) {
    let connection = establish_connection();
    let tag_id = args.get(2).unwrap();
    let asset_id = args.get(3).unwrap();
    db::tag::add_asset(&connection, tag_id, asset_id);
}
