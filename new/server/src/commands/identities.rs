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

// pub fn remove_coin(args: &[String]) {
//     let id = args.get(2).unwrap();
//     let connection = establish_connection();
//     db::remove_coin(&connection, id);
// }

pub fn show_identities(_args: &[String]) {
    let connection = establish_connection();
    let identities = db::get_identities(&connection);
    for identity in identities {
        println!("ID: {}", identity.id);
        println!("----------");
        println!("Name: {}", identity.name);
        // let sessions = db::fetch_sessions(&connection, &identity.id);
        // println!("{:#?}", sessions);
    }
}
