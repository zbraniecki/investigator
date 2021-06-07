use crate::api::MarketAssetInfo;
use crate::db;
use crate::models::{Market, MarketAsset};
use crate::models::{NewMarket, NewMarketAsset};
use diesel::prelude::*;

pub fn create_market(conn: &PgConnection, id: &str, name: &str) {
    use crate::db::schema::markets;

    let new_market = NewMarket { id, name };

    diesel::insert_into(markets::table)
        .values(&new_market)
        .execute(conn)
        .expect("Error saving new market");
}

pub fn remove_market(conn: &PgConnection, delete_id: &str) {
    use crate::db::schema::markets::dsl::*;

    let _num_deleted = diesel::delete(markets.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting market");
}

pub fn get_markets(conn: &PgConnection) -> Vec<Market> {
    use crate::db::schema::markets::dsl::*;

    let results = markets
        .order(id.desc())
        .load::<Market>(conn)
        .expect("Error loading markets");
    results
}

pub fn set_market_assets(conn: &PgConnection, market_id: &str, asset_list: &[MarketAssetInfo]) {
    use crate::db::schema::market_assets;

    let class = "crypto";

    db::remove_market_assets(conn, market_id);
    for a in asset_list.iter() {
        if !db::get_asset(conn, &a.id).is_some() {
            db::create_asset(conn, &a.id, class, None, None);
        }
        let new_asset = NewMarketAsset {
            market: market_id,
            asset: &a.id,
            class,
        };

        diesel::insert_into(market_assets::table)
            .values(&new_asset)
            .execute(conn)
            .expect("Error saving new asset");
    }
}

pub fn remove_market_assets(conn: &PgConnection, market_id: &str) {
    use crate::db::schema::market_assets::dsl::*;

    let _num_deleted = diesel::delete(market_assets.filter(market.eq(market_id)))
        .execute(conn)
        .expect("Error deleting market assets");
}

pub fn get_market_assets(conn: &PgConnection, market_id: &str) -> Vec<MarketAsset> {
    use crate::db::schema::market_assets::dsl::*;

    let results = market_assets
        .filter(market.eq(market_id))
        .load::<MarketAsset>(conn)
        .expect("Error loading market assets");
    results
}
