use super::models::{Service, NewService, Wallet, NewWallet};
use diesel::prelude::*;

pub mod service {
    use super::*;

    pub fn create(conn: &PgConnection, id: &str, name: &str) {
        use crate::db::schema::services;

        let new_service = NewService { id, name, url: None, owner: None };

        diesel::insert_into(services::table)
            .values(&new_service)
            .execute(conn)
            .expect("Error inserting service");
    }

    // pub fn get(conn: &PgConnection, service_id: &str) -> Option<Service> {
    //     use crate::db::schema::services::dsl::*;

    //     let results = services
    //         .filter(id.eq(service_id))
    //         .load::<Service>(conn)
    //         .expect("Error loading service");
    //     results.get(0).cloned()
    // }

    pub fn delete(conn: &PgConnection, delete_id: &str) {
        use crate::db::schema::services::dsl::*;

        let _num_deleted = diesel::delete(services.filter(id.eq(delete_id)))
            .execute(conn)
            .expect("Error deleting service");
    }

    pub fn filter(conn: &PgConnection) -> Vec<Service> {
        use crate::db::schema::services::dsl::*;

        let results = services
            .order(id.desc())
            .load::<Service>(conn)
            .expect("Error loading services");
        results
    }
}

pub mod wallet {
    use super::*;

    pub fn create(conn: &PgConnection, id: &str, name: &str) {
        use crate::db::schema::wallets;

        let new_wallet = NewWallet { id, name: Some(name), url: None, service: None, owner: None };

        diesel::insert_into(wallets::table)
            .values(&new_wallet)
            .execute(conn)
            .expect("Error inserting wallet");
    }

    // pub fn get(conn: &PgConnection, wallet_id: &str) -> Option<Wallet> {
    //     use crate::db::schema::wallets::dsl::*;

    //     let results = wallets
    //         .filter(id.eq(wallet_id))
    //         .load::<Wallet>(conn)
    //         .expect("Error loading wallet");
    //     results.get(0).cloned()
    // }

    pub fn delete(conn: &PgConnection, delete_id: &str) {
        use crate::db::schema::wallets::dsl::*;

        let _num_deleted = diesel::delete(wallets.filter(id.eq(delete_id)))
            .execute(conn)
            .expect("Error deleting wallet");
    }

    pub fn filter(conn: &PgConnection) -> Vec<Wallet> {
        use crate::db::schema::wallets::dsl::*;

        let results = wallets
            .order(id.desc())
            .load::<Wallet>(conn)
            .expect("Error loading wallets");
        results
    }
}
