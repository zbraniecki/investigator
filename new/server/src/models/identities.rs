use crate::db::schema::identities;

#[derive(Queryable)]
pub struct Identity {
    pub id: i32,
    pub name: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "identities"]
pub struct NewIdentity<'a> {
    pub name: &'a str,
    pub password: &'a str,
}
