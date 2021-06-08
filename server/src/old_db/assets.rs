use crate::api::AssetInfo;
use crate::models::{Asset, AssetClass};
use crate::models::{NewAsset, NewAssetClass};
use diesel::prelude::*;

pub fn create_asset(
    conn: &PgConnection,
    id: &str,
    class: &str,
    symbol: Option<&str>,
    name: Option<&str>,
) {
    use crate::db::schema::assets;

    let new_asset = NewAsset {
        id,
        class,
        symbol,
        name,
    };

    diesel::insert_into(assets::table)
        .values(&new_asset)
        .execute(conn)
        .expect("Error saving new asset");
}

pub fn remove_asset(conn: &PgConnection, delete_id: &str) {
    use crate::db::schema::assets::dsl::*;

    let _num_deleted = diesel::delete(assets.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting asset");
}

pub fn get_asset(conn: &PgConnection, asset_id: &str) -> Option<Asset> {
    use crate::db::schema::assets::dsl::*;

    let results = assets
        .filter(id.eq(asset_id))
        .load::<Asset>(conn)
        .expect("Error loading assets");
    results.get(0).cloned()
}

pub fn get_assets(conn: &PgConnection) -> Vec<Asset> {
    use crate::db::schema::assets::dsl::*;

    let results = assets
        .order(id.desc())
        .load::<Asset>(conn)
        .expect("Error loading assets");
    results
}

pub fn create_asset_class(conn: &PgConnection, id: &str, name: &str) {
    use crate::db::schema::asset_classes;

    let new_asset_class = NewAssetClass { id, name };

    diesel::insert_into(asset_classes::table)
        .values(&new_asset_class)
        .execute(conn)
        .expect("Error saving new asset class");
}

pub fn remove_asset_class(conn: &PgConnection, delete_id: &str) {
    use crate::db::schema::asset_classes::dsl::*;

    let _num_deleted = diesel::delete(asset_classes.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting asset class");
}

pub fn get_asset_classes(conn: &PgConnection) -> Vec<AssetClass> {
    use crate::db::schema::asset_classes::dsl::*;

    let results = asset_classes
        .order(id.desc())
        .load::<AssetClass>(conn)
        .expect("Error loading asset classes");
    results
}
