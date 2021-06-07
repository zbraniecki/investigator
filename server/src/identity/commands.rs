use super::db;
use crate::db::establish_connection;

pub fn get_prefix() -> &'static str {
    "identity"
}

pub fn get_list() -> Vec<&'static str> {
    vec!["create", "read", "update", "delete", "filter"]
}

pub fn handle_command(cmd: &str, args: &[String]) {
    match cmd {
        "create" => create_identity(args),
        "delete" => delete_identity(args),
        "filter" => filter_identities(args),
        _ => {}
    }
}

pub fn create_identity(args: &[String]) {
    let connection = establish_connection();
    let name = args.get(2).unwrap();
    let password = args.get(3).unwrap();
    db::create_identity(&connection, name, password);
}

pub fn delete_identity(args: &[String]) {
    let name = args.get(2).unwrap();
    let connection = establish_connection();
    let identity = db::get_identity_by_name(&connection, &name).unwrap();
    db::delete_identity(&connection, identity.id);
}

pub fn filter_identities(_args: &[String]) {
    let connection = establish_connection();
    let identities = db::filter_identities(&connection);
    for identity in identities {
        println!("ID: {}", identity.id);
        println!("----------");
        println!("Name: {}", identity.name);
        let sessions = db::get_sessions(&connection, identity.id);
        println!("{:#?}", sessions);
    }
}
