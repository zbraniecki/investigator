use super::super::models::{Category, NewCategory, NewTagCategory, TagCategory};
use diesel::prelude::*;

pub fn create(conn: &PgConnection, id: &str) {
    use crate::db::schema::asset_categories;

    let new_category = NewCategory { id, owner: None };

    diesel::insert_into(asset_categories::table)
        .values(&new_category)
        .execute(conn)
        .expect("Error saving new asset category");
}

pub fn delete(conn: &PgConnection, delete_id: &str) {
    use crate::db::schema::asset_categories::dsl::*;

    let _num_deleted = diesel::delete(asset_categories.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting asset category");
}

pub fn _get(conn: &PgConnection, asset_id: &str) -> Option<Category> {
    use crate::db::schema::asset_categories::dsl::*;

    let results = asset_categories
        .filter(id.eq(asset_id))
        .load::<Category>(conn)
        .expect("Error loading asset category");
    results.get(0).cloned()
}

pub fn filter(conn: &PgConnection) -> Vec<Category> {
    use crate::db::schema::asset_categories::dsl::*;

    let results = asset_categories
        .order(id.desc())
        .load::<Category>(conn)
        .expect("Error loading asset categories");
    results
}

pub fn add_tag(conn: &PgConnection, tag_id: &str, cat_id: &str) {
    use crate::db::schema::asset_tag_categories;

    let new_tag_cat = NewTagCategory {
        tag: tag_id,
        category: cat_id,
    };

    diesel::insert_into(asset_tag_categories::table)
        .values(&new_tag_cat)
        .execute(conn)
        .expect("Error saving new asset tag-category");
}

pub fn get_tags(conn: &PgConnection, cat_id: &str) -> Vec<TagCategory> {
    use crate::db::schema::asset_tag_categories::dsl::*;

    let results = asset_tag_categories
        .filter(category.eq(cat_id))
        .order(tag.desc())
        .load::<TagCategory>(conn)
        .expect("Error loading asset categories");
    results
}
