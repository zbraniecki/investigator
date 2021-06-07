use super::establish_connection;
// use crate::api;
use crate::db;
// use std::time::Duration;
// use tokio::time::delay_for;

pub fn add_identity(args: &[String]) {
    let name = args.get(2).unwrap();
    let password = args.get(3).unwrap();
    let connection = establish_connection();
    db::create_identity(&connection, name, password);
}

pub fn remove_identity(args: &[String]) {
    let name = args.get(2).unwrap();
    let connection = establish_connection();
    let identity = db::get_identity_by_name(&connection, &name).unwrap();
    db::remove_identity(&connection, identity.id);
}

pub fn show_identities(_args: &[String]) {
    let connection = establish_connection();
    let identities = db::get_identities(&connection);
    for identity in identities {
        println!("ID: {}", identity.id);
        println!("----------");
        println!("Name: {}", identity.name);
        let sessions = db::get_sessions(&connection, identity.id);
        println!("{:#?}", sessions);
    }
}

pub fn get_identity_by_name(conn: &PgConnection, get_name: &str) -> Option<Identity> {
    use crate::db::schema::identities::dsl::*;

    let results = identities
        .filter(name.eq(get_name))
        .load::<Identity>(conn)
        .expect("Error loading coins");
    results.get(0).cloned()
}

