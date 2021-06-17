use super::models::{NewPortfolio, NewPortfolioAsset, Portfolio, PortfolioAsset};
use diesel::prelude::*;

pub mod portfolio {
    use super::*;

    pub fn create(conn: &PgConnection, slug: &str, name: &str) {
        use crate::db::schema::portfolios;

        let new_portfolio = NewPortfolio {
            slug,
            name: Some(name),
            owner: None,
        };

        diesel::insert_into(portfolios::table)
            .values(&new_portfolio)
            .execute(conn)
            .expect("Error inserting portfolio");
    }

    pub fn get(conn: &PgConnection, get_id: i64) -> Option<Portfolio> {
        use crate::db::schema::portfolios::dsl::*;

        let results = portfolios
            .filter(id.eq(get_id))
            .load::<Portfolio>(conn)
            .expect("Error loading portfolio");
        results.get(0).cloned()
    }

    pub fn delete(conn: &PgConnection, delete_id: i64) {
        use crate::db::schema::portfolios::dsl::*;

        let _num_deleted = diesel::delete(portfolios.filter(id.eq(delete_id)))
            .execute(conn)
            .expect("Error deleting portfolio");
    }

    pub fn filter(conn: &PgConnection) -> Vec<Portfolio> {
        use crate::db::schema::portfolios::dsl::*;

        let results = portfolios
            .order(id.desc())
            .load::<Portfolio>(conn)
            .expect("Error loading portfolios");
        results
    }
}

pub mod portfolio_assets {
    use super::*;

    pub fn create(conn: &PgConnection, portfolio: i64, asset: &str) {
        use crate::db::schema::portfolio_assets;

        let new_portfolio_asset = NewPortfolioAsset { portfolio, asset };

        diesel::insert_into(portfolio_assets::table)
            .values(&new_portfolio_asset)
            .execute(conn)
            .expect("Error inserting portfolio_asset");
    }

    pub fn clear(conn: &PgConnection, portfolio_id: i64) {
        use crate::db::schema::portfolio_assets::dsl::*;

        let _num_deleted = diesel::delete(portfolio_assets.filter(portfolio.eq(portfolio_id)))
            .execute(conn)
            .expect("Error deleting assets from portfolio");
    }

    pub fn filter(conn: &PgConnection, portfolio_id: i64) -> Vec<PortfolioAsset> {
        use crate::db::schema::portfolio_assets::dsl::*;

        let results = portfolio_assets
            .filter(portfolio.eq(portfolio_id))
            .load::<PortfolioAsset>(conn)
            .expect("Error loading portfolio_assets");
        results
    }
}
