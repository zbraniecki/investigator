use crate::schema::coins;

#[derive(Queryable)]
pub struct Coin {
    pub id: String,
    pub symbol: Option<String>,
    pub name: Option<String>,
}

#[derive(Insertable)]
#[table_name = "coins"]
pub struct NewCoin<'a> {
    pub id: &'a str,
    pub symbol: Option<&'a str>,
    pub name: Option<&'a str>,
}
