use super::db;
use crate::db::establish_connection;

pub fn get_prefix() -> &'static str {
    "identity"
}

pub fn get_list() -> Vec<&'static str> {
    vec!["create", "read", "update", "delete", "filter"]
}

pub fn handle_command(cmd: &str, args: &[String]) -> bool {
    match cmd {
        "create" => create(args),
        "delete" => delete(args),
        "filter" => filter(args),
        _ => {
            return false;
        }
    }
    true
}

pub fn create(args: &[String]) {
    let connection = establish_connection();
    let name = args.get(2).unwrap();
    let password = args.get(3).unwrap();
    db::identity::create(&connection, name, password);
}

pub fn delete(args: &[String]) {
    let name = args.get(2).unwrap();
    let connection = establish_connection();
    let identity = db::identity::get_by_name(&connection, &name).unwrap();
    db::identity::delete(&connection, identity.id);
}

pub fn filter(_args: &[String]) {
    let connection = establish_connection();
    let identities = db::identity::filter(&connection);
    for identity in identities {
        println!("ID: {}", identity.id);
        println!("----------");
        println!("Name: {}", identity.name);
        let sessions = db::session::get(&connection, identity.id);
        println!("{:#?}", sessions);
    }
}
