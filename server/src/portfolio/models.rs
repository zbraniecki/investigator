use crate::db::schema::{portfolio_assets, portfolios};

#[derive(Queryable, Clone, Debug)]
pub struct Portfolio {
    pub id: i32,
    pub slug: String,
    pub name: Option<String>,
    pub owner: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "portfolios"]
pub struct NewPortfolio<'a> {
    pub slug: &'a str,
    pub name: Option<&'a str>,
    pub owner: Option<i32>,
}

#[derive(Queryable, Clone, Debug)]
pub struct PortfolioAsset {
    pub portfolio: i32,
    pub asset: String,
}

#[derive(Insertable)]
#[table_name = "portfolio_assets"]
pub struct NewPortfolioAsset<'a> {
    pub portfolio: i32,
    pub asset: &'a str,
}
