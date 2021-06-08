use super::super::models::{AssetTag, NewAssetTag, NewTag, Tag};
use diesel::prelude::*;

pub fn create(conn: &PgConnection, id: &str) {
    use crate::db::schema::asset_tags;

    let new_tag = NewTag { id, owner: None };

    diesel::insert_into(asset_tags::table)
        .values(&new_tag)
        .execute(conn)
        .expect("Error saving new asset category");
}

pub fn delete(conn: &PgConnection, delete_id: &str) {
    use crate::db::schema::asset_tags::dsl::*;

    let _num_deleted = diesel::delete(asset_tags.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting asset category");
}

pub fn _get(conn: &PgConnection, asset_id: &str) -> Option<Tag> {
    use crate::db::schema::asset_tags::dsl::*;

    let results = asset_tags
        .filter(id.eq(asset_id))
        .load::<Tag>(conn)
        .expect("Error loading asset category");
    results.get(0).cloned()
}

pub fn filter(conn: &PgConnection) -> Vec<Tag> {
    use crate::db::schema::asset_tags::dsl::*;

    let results = asset_tags
        .order(id.desc())
        .load::<Tag>(conn)
        .expect("Error loading asset categories");
    results
}

pub fn add_asset(conn: &PgConnection, tag_id: &str, asset_id: &str) {
    use crate::db::schema::asset_tags_intermediate;

    let new_asset_tag = NewAssetTag {
        tag: tag_id,
        asset: asset_id,
    };

    diesel::insert_into(asset_tags_intermediate::table)
        .values(&new_asset_tag)
        .execute(conn)
        .expect("Error saving new asset tag");
}

pub fn get_assets(conn: &PgConnection, tag_id: &str) -> Vec<AssetTag> {
    use crate::db::schema::asset_tags_intermediate::dsl::*;

    let results = asset_tags_intermediate
        .filter(tag.eq(tag_id))
        .order(tag.desc())
        .load::<AssetTag>(conn)
        .expect("Error loading asset tags");
    results
}
