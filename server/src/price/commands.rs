use super::api;
use super::db;
use crate::db::establish_connection;
use chrono::{NaiveDate, NaiveDateTime};

pub fn get_prefix() -> &'static str {
    "prices"
}

pub fn get_list() -> Vec<&'static str> {
    vec!["fetch"]
}

pub async fn handle_command(cmd: &str, args: &[String]) -> bool {
    match cmd {
        "fetch" => fetch(args).await,
        _ => {
            return false;
        }
    }
    true
}

// pub fn create(args: &[String]) {
//     let connection = establish_connection();
//     let name = args.get(2).unwrap();
//     let password = args.get(3).unwrap();
//     db::identity::create(&connection, name, password);
// }

// pub fn delete(args: &[String]) {
//     let name = args.get(2).unwrap();
//     let connection = establish_connection();
//     let identity = db::identity::get_by_name(&connection, &name).unwrap();
//     db::identity::delete(&connection, identity.id);
// }

// pub fn filter(_args: &[String]) {
//     let connection = establish_connection();
//     let identities = db::identity::filter(&connection);
//     for identity in identities {
//         println!("ID: {}", identity.id);
//         println!("----------");
//         println!("Name: {}", identity.name);
//         let sessions = db::session::get(&connection, identity.id);
//         println!("{:#?}", sessions);
//     }
// }

pub async fn fetch(args: &[String]) {
    let id = args.get(2).unwrap();
    let target = "usd";
    let asset_prices = api::fetch_asset_prices(&id, target).await.unwrap();

    let mut result: Vec<(NaiveDate, f64)> = vec![];

    let mut current_date = None;
    let mut values = vec![];
    for (ts, value) in &asset_prices.prices {
        // Something about 13 digits epoch vs 10 digits epoch and the
        // last 3 digits being microseconds? Not sure, but it works for now.
        let dt = NaiveDateTime::from_timestamp(*ts / 1000, 0);
        let new_date = dt.date();
        if let Some(date) = current_date {
            if date == new_date {
                values.push(value);
                continue;
            }
            let len = values.len();
            let sum: f64 = Iterator::sum(values.into_iter());
            let avg = sum / (len as f64);
            result.push((new_date, avg));
        }
        current_date = Some(new_date);
        values = vec![value];
    }
    let connection = establish_connection();
    db::clean_asset_prices(&connection, id);
    db::set_asset_prices(&connection, id, target, &result);
}
