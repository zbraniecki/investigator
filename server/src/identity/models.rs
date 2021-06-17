use crate::db::schema::{identities, sessions};
use chrono::prelude::*;

#[derive(Queryable, Clone, Debug)]
pub struct Identity {
    pub id: i64,
    pub name: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "identities"]
pub struct NewIdentity<'a> {
    pub name: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Clone, Debug)]
pub struct Session {
    pub id: i64,
    pub identity: i64,
    pub expires: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub identity: i64,
    pub expires: Option<NaiveDateTime>,
}
