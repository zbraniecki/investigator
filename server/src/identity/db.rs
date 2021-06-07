use super::models::{Identity, Session};
use super::models::{NewIdentity, NewSession};
use diesel::prelude::*;

pub mod identity {
    use super::*;

    pub fn create(conn: &PgConnection, name: &str, password: &str) {
        use crate::db::schema::identities;

        // XXX: Encrypt the password before anyone ever uses this! :)
        let new_identity = NewIdentity { name, password };

        diesel::insert_into(identities::table)
            .values(&new_identity)
            .execute(conn)
            .expect("Error inserting identity");
    }

    pub fn get_by_name(conn: &PgConnection, get_name: &str) -> Option<Identity> {
        use crate::db::schema::identities::dsl::*;

        let results = identities
            .filter(name.eq(get_name))
            .load::<Identity>(conn)
            .expect("Error loading coins");
        results.get(0).cloned()
    }

    pub fn delete(conn: &PgConnection, delete_id: i32) {
        use crate::db::schema::identities::dsl::*;

        let _num_deleted = diesel::delete(identities.filter(id.eq(delete_id)))
            .execute(conn)
            .expect("Error deleting identity");
    }

    pub fn filter(conn: &PgConnection) -> Vec<Identity> {
        use crate::db::schema::identities::dsl::*;

        let results = identities
            .order(id.desc())
            .load::<Identity>(conn)
            .expect("Error loading coins");
        results
    }
}

pub mod session {
    use super::*;

    pub fn get(conn: &PgConnection, identity_id: i32) -> Vec<Session> {
        use crate::db::schema::sessions::dsl::*;

        let results = sessions
            .filter(identity.eq(identity_id))
            .order(id.desc())
            .load::<Session>(conn)
            .expect("Error loading coins");
        results
    }
}
