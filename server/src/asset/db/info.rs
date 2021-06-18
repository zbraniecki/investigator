use super::super::models::AssetInfo;
use diesel::prelude::*;

pub fn create(conn: &PgConnection, info: AssetInfo) {
    use crate::db::schema::assets_info;

    diesel::insert_into(assets_info::table)
        .values(&info)
        .execute(conn)
        .expect("Error saving new asset info");
}

pub fn delete(conn: &PgConnection, delete_id: &str) {
    use crate::db::schema::assets_info::dsl::*;

    let _num_deleted = diesel::delete(assets_info.filter(asset.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting asset info");
}

pub fn get(conn: &PgConnection, asset_id: &str) -> Option<AssetInfo> {
    use crate::db::schema::assets_info::dsl::*;

    let results = assets_info
        .filter(asset.eq(asset_id))
        .load::<AssetInfo>(conn)
        .expect("Error loading asset info");
    results.get(0).cloned()
}

pub fn filter(conn: &PgConnection, asset_ids: Vec<&str>) -> Vec<AssetInfo> {
    use crate::db::schema::assets_info::dsl::*;

    let results = assets_info
        .filter(asset.eq_any(asset_ids))
        .load::<AssetInfo>(conn)
        .expect("Error loading asset info");
    results
}
