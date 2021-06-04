use crate::db::schema::{asset_classes, assets};

#[derive(Queryable, Clone, Debug)]
pub struct Asset {
    pub id: String,
    pub class: String,
    pub symbol: Option<String>,
    pub name: Option<String>,
}

#[derive(Insertable)]
#[table_name = "assets"]
pub struct NewAsset<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub symbol: Option<&'a str>,
    pub name: Option<&'a str>,
}

#[derive(Queryable)]
pub struct AssetClass {
    pub id: String,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "asset_classes"]
pub struct NewAssetClass<'a> {
    pub id: &'a str,
    pub name: &'a str,
}
