use crate::db::schema::assets;

#[derive(Queryable, Clone, Debug)]
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
