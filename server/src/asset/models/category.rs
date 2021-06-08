use crate::db::schema::asset_categories;

#[derive(Queryable, Clone, Debug)]
pub struct Category {
    pub id: String,
    pub owner: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "asset_categories"]
pub struct NewCategory<'a> {
    pub id: &'a str,
    pub owner: Option<i32>,
}
