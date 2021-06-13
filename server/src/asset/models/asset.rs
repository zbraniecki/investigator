use crate::db::schema::assets;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub symbol: Option<String>,
    pub name: Option<String>,
}

#[derive(Insertable)]
#[table_name = "assets"]
pub struct NewAsset<'a> {
    pub id: &'a str,
    pub symbol: Option<&'a str>,
    pub name: Option<&'a str>,
}
