// use crate::api::CoinInfo;
use crate::models::Identity;
use crate::models::NewIdentity;
use diesel::prelude::*;

pub fn create_identity(conn: &PgConnection, name: &str, password: &str) {
    use crate::db::schema::identities;

    let new_identity = NewIdentity { name, password };

    diesel::insert_into(identities::table)
        .values(&new_identity)
        .execute(conn)
        .expect("Error inserting identity");
}

pub fn remove_identity(conn: &PgConnection, delete_id: i32) {
    use crate::db::schema::identities::dsl::*;

    let _num_deleted = diesel::delete(identities.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting identity");
}

pub fn get_identities(conn: &PgConnection) -> Vec<Identity> {
    use crate::db::schema::identities::dsl::*;

    let results = identities
        .order(id.desc())
        .load::<Identity>(conn)
        .expect("Error loading coins");
    results
}

pub fn get_identity(conn: &PgConnection, get_id: i32) -> Option<Identity> {
    use crate::db::schema::identities::dsl::*;

    let results = identities
        .filter(id.eq(get_id))
        .load::<Identity>(conn)
        .expect("Error loading coins");
    results.get(0).cloned()
}

pub fn get_identity_by_name(conn: &PgConnection, get_name: &str) -> Option<Identity> {
    use crate::db::schema::identities::dsl::*;

    let results = identities
        .filter(name.eq(get_name))
        .load::<Identity>(conn)
        .expect("Error loading coins");
    results.get(0).cloned()
}
