use super::super::models::Asset;
use super::super::models::NewAsset;
use diesel::prelude::*;

pub fn create(conn: &PgConnection, id: &str, symbol: Option<&str>, name: Option<&str>) {
    use crate::db::schema::assets;

    let new_asset = NewAsset { id, symbol, name };

    diesel::insert_into(assets::table)
        .values(&new_asset)
        .execute(conn)
        .expect("Error saving new asset");
}

pub fn delete(conn: &PgConnection, delete_id: &str) {
    use crate::db::schema::assets::dsl::*;

    let _num_deleted = diesel::delete(assets.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting asset");
}

pub fn _get(conn: &PgConnection, asset_id: &str) -> Option<Asset> {
    use crate::db::schema::assets::dsl::*;

    let results = assets
        .filter(id.eq(asset_id))
        .load::<Asset>(conn)
        .expect("Error loading assets");
    results.get(0).cloned()
}

pub fn filter(conn: &PgConnection) -> Vec<Asset> {
    use crate::db::schema::assets::dsl::*;

    let results = assets
        .order(id.desc())
        .load::<Asset>(conn)
        .expect("Error loading assets");
    results
}
