use crate::db::schema::{asset_tag_categories, asset_tags, asset_tags_intermediate};

#[derive(Queryable, Clone, Debug)]
pub struct Tag {
    pub id: String,
    pub owner: Option<i64>,
}

#[derive(Insertable)]
#[table_name = "asset_tags"]
pub struct NewTag<'a> {
    pub id: &'a str,
    pub owner: Option<i64>,
}

#[derive(Queryable, Clone, Debug)]
pub struct TagCategory {
    pub tag: String,
    pub category: String,
}

#[derive(Insertable)]
#[table_name = "asset_tag_categories"]
pub struct NewTagCategory<'a> {
    pub tag: &'a str,
    pub category: &'a str,
}

#[derive(Queryable, Clone, Debug)]
pub struct AssetTag {
    pub tag: String,
    pub asset: String,
}

#[derive(Insertable)]
#[table_name = "asset_tags_intermediate"]
pub struct NewAssetTag<'a> {
    pub tag: &'a str,
    pub asset: &'a str,
}
