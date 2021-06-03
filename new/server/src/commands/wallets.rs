use super::establish_connection;
// use crate::api;
use crate::db;
// use std::time::Duration;
// use tokio::time::delay_for;
use chrono::NaiveDateTime;

pub fn add_wallet(args: &[String]) {
    let id = args.get(2).unwrap();
    let name = args.get(3).unwrap();
    let connection = establish_connection();
    db::create_wallet(&connection, id, name, None);
}

pub fn remove_wallet(args: &[String]) {
    let id = args.get(2).unwrap();
    let connection = establish_connection();
    db::remove_wallet(&connection, id);
}

pub fn show_wallets(_args: &[String]) {
    let connection = establish_connection();
    let wallets = db::get_wallets(&connection);
    for wallet in wallets {
        println!("----------");
        println!("ID: {}", wallet.id);
        println!("Name: {}", wallet.name);
        let passive_incomes = db::get_passive_incomes(&connection, &wallet.id);
        println!("{:#?}", passive_incomes);
    }
}

pub fn add_passive_income(args: &[String]) {
    let wallet_id = args.get(2).unwrap();
    let coin = args.get(3).unwrap();
    let kind = args.get(4).unwrap();
    let apy: f64 = args.get(5).unwrap().parse().unwrap();
    let apy_upper: Option<f64> = args.get(6).map(|a| a.parse().unwrap());
    let start_date: Option<NaiveDateTime> = args.get(7).map(|a| a.parse().unwrap());
    let end_date: Option<NaiveDateTime> = args.get(8).map(|a| a.parse().unwrap());
    let connection = establish_connection();
    db::create_passive_income(
        &connection,
        wallet_id,
        coin,
        kind,
        apy,
        apy_upper,
        start_date,
        end_date,
    );
}

pub fn remove_passive_income(args: &[String]) {
    let wallet_id = args.get(2).unwrap();
    let coin_id = args.get(3).unwrap();
    let kind_id = args.get(4).unwrap();
    let connection = establish_connection();
    db::remove_passive_income(&connection, wallet_id, coin_id, kind_id, None);
}

pub fn clear_passive_incomes(args: &[String]) {
    let wallet_id = args.get(2).unwrap();
    let connection = establish_connection();
    db::clear_passive_incomes(&connection, wallet_id);
}
