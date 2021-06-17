use crate::db::schema::{services, wallets};

#[derive(Queryable, Clone, Debug)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub owner: Option<i64>,
}

#[derive(Insertable)]
#[table_name = "services"]
pub struct NewService<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub url: Option<&'a str>,
    pub owner: Option<i64>,
}

#[derive(Queryable, Clone, Debug)]
pub struct Wallet {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub service: Option<String>,
    pub owner: Option<i64>,
}

#[derive(Insertable)]
#[table_name = "wallets"]
pub struct NewWallet<'a> {
    pub id: &'a str,
    pub name: Option<&'a str>,
    pub url: Option<&'a str>,
    pub service: Option<&'a str>,
    pub owner: Option<i64>,
}
