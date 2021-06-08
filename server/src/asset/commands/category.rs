use super::super::db;
use crate::db::establish_connection;

pub fn get_prefix() -> &'static str {
    "asset_category"
}

pub fn get_list() -> Vec<&'static str> {
    vec!["create", "read", "update", "delete", "filter", "addTag"]
}

pub fn handle_command(cmd: &str, args: &[String]) -> bool {
    match cmd {
        "create" => create(args),
        "delete" => delete(args),
        "filter" => filter(args),
        "addTag" => add_tag(args),
        _ => {
            return false;
        }
    }
    true
}

pub fn create(args: &[String]) {
    let connection = establish_connection();
    let id = args.get(2).unwrap();
    db::category::create(&connection, id);
}

pub fn delete(args: &[String]) {
    let connection = establish_connection();
    let id = args.get(2).unwrap();
    db::category::delete(&connection, id);
}

pub fn filter(_args: &[String]) {
    let connection = establish_connection();
    let categories = db::category::filter(&connection);
    for cat in categories {
        println!("ID: {}", cat.id);
        let tags = db::category::get_tags(&connection, &cat.id);
        println!("{:#?}", tags);
    }
}

pub fn add_tag(args: &[String]) {
    let connection = establish_connection();
    let tag_id = args.get(2).unwrap();
    let category_id = args.get(3).unwrap();
    db::category::add_tag(&connection, tag_id, category_id);
}
