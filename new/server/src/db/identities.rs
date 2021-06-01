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

// pub fn remove_coin(conn: &PgConnection, delete_id: &str) {
//     use crate::db::schema::coins::dsl::*;

//     let _num_deleted = diesel::delete(coins.filter(id.eq(delete_id)))
//         .execute(conn)
//         .expect("Error deleting coins");
// }

pub fn get_identities(conn: &PgConnection) -> Vec<Identity> {
    use crate::db::schema::identities::dsl::*;

    let results = identities
        .order(id.desc())
        .load::<Identity>(conn)
        .expect("Error loading coins");
    results
}
