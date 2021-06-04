use crate::db::schema::{market_assets, markets};

#[derive(Queryable, Debug)]
pub struct Market {
    pub id: String,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "markets"]
pub struct NewMarket<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

#[derive(Queryable, Debug)]
pub struct MarketAsset {
    pub market: String,
    pub asset: String,
    pub class: String,
}

#[derive(Insertable, Debug)]
#[table_name = "market_assets"]
pub struct NewMarketAsset<'a> {
    pub market: &'a str,
    pub asset: &'a str,
    pub class: &'a str,
}
