use crate::db::schema::{passive_incomes, wallets};
use chrono::prelude::*;

#[derive(Queryable, Clone, Debug)]
pub struct Wallet {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Insertable)]
#[table_name = "wallets"]
pub struct NewWallet<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub url: Option<&'a str>,
}

#[derive(Queryable, Clone, Debug)]
pub struct PassiveIncome {
    pub wallet: String,
    pub coin: String,
    pub kind: String,
    pub apy: f64,
    pub apy_upper_bound: Option<f64>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "passive_incomes"]
pub struct NewPassiveIncome<'a> {
    pub wallet: &'a str,
    pub coin: &'a str,
    pub kind: &'a str,
    pub apy: f64,
    pub apy_upper_bound: Option<f64>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}
