use super::schema::{coins, prices};
use chrono;
use chrono::prelude::*;

#[derive(Queryable)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="coins"]
pub struct NewCoin<'a> {
    pub id: &'a str,
    pub symbol: &'a str,
    pub name: &'a str,
}

#[derive(Queryable, Clone)]
pub struct Price {
    pub base: String,
    pub target: String,
    pub ts: NaiveDateTime,
    pub value: f64,
}

#[derive(Insertable)]
#[table_name="prices"]
pub struct NewPrice<'a> {
    pub base: &'a str,
    pub target: &'a str,
    pub ts: NaiveDateTime,
    pub value: f64,
}

