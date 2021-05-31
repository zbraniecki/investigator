use crate::schema::prices;
use chrono;
use chrono::prelude::*;

#[derive(Queryable, Clone, Debug)]
pub struct Price {
    pub base: String,
    pub target: String,
    pub ts: NaiveDateTime,
    pub value: f64,
}

#[derive(Insertable, Debug)]
#[table_name = "prices"]
pub struct NewPrice<'a> {
    pub base: &'a str,
    pub target: &'a str,
    pub ts: NaiveDateTime,
    pub value: f64,
}
