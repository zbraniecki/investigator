use super::super::db;
use crate::db::establish_connection;

pub fn get_prefix() -> &'static str {
    "service"
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
    db::service::create(&connection, name, password);
}

pub fn delete(args: &[String]) {
    let id = args.get(2).unwrap();
    let connection = establish_connection();
    db::service::delete(&connection, id);
}

pub fn filter(_args: &[String]) {
    let connection = establish_connection();
    let services = db::service::filter(&connection);
    for service in services {
        println!("ID: {}", service.id);
        println!("Name: {}", service.name);
    }
}
