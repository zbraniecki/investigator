use crate::models::{NewPassiveIncome, NewWallet};
use crate::models::{PassiveIncome, Wallet};
use chrono::prelude::*;
use diesel::prelude::*;

pub fn create_wallet(conn: &PgConnection, id: &str, name: &str, url: Option<&str>) {
    use crate::db::schema::wallets;

    let new_wallet = NewWallet { id, name, url };

    diesel::insert_into(wallets::table)
        .values(&new_wallet)
        .execute(conn)
        .expect("Error inserting wallet");
}

pub fn remove_wallet(conn: &PgConnection, delete_id: &str) {
    use crate::db::schema::wallets::dsl::*;

    let _num_deleted = diesel::delete(wallets.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting wallet");
}

pub fn get_wallets(conn: &PgConnection) -> Vec<Wallet> {
    use crate::db::schema::wallets::dsl::*;

    let results = wallets
        .order(id.desc())
        .load::<Wallet>(conn)
        .expect("Error loading wallets");
    results
}

pub fn get_wallet(conn: &PgConnection, get_id: &str) -> Option<Wallet> {
    use crate::db::schema::wallets::dsl::*;

    let results = wallets
        .filter(id.eq(get_id))
        .load::<Wallet>(conn)
        .expect("Error loading wallet");
    results.get(0).cloned()
}

pub fn create_passive_income(
    conn: &PgConnection,
    wallet_id: &str,
    coin: &str,
    kind: &str,
    apy: f64,
    apy_upper: Option<f64>,
    start_date: Option<NaiveDateTime>,
    end_date: Option<NaiveDateTime>,
) {
    use crate::db::schema::passive_incomes;

    let new_session = NewPassiveIncome {
        wallet: wallet_id,
        coin,
        kind,
        apy,
        apy_upper_bound: apy_upper,
        start_date,
        end_date,
    };

    diesel::insert_into(passive_incomes::table)
        .values(&new_session)
        .execute(conn)
        .expect("Error inserting session");
}

pub fn get_passive_incomes(conn: &PgConnection, wallet_id: &str) -> Vec<PassiveIncome> {
    use crate::db::schema::passive_incomes::dsl::*;

    let results = passive_incomes
        .filter(wallet.eq(wallet_id))
        .order(start_date.desc())
        .load::<PassiveIncome>(conn)
        .expect("Error loading coins");
    results
}

pub fn remove_passive_income(
    conn: &PgConnection,
    wallet_id: &str,
    coin_id: &str,
    kind_id: &str,
    _date: Option<NaiveDateTime>,
) {
    use crate::db::schema::passive_incomes::dsl::*;

    let _num_deleted = diesel::delete(
        passive_incomes
            .filter(wallet.eq(wallet_id))
            .filter(coin.eq(coin_id))
            .filter(kind.eq(kind_id)),
    )
    .execute(conn)
    .expect("Error deleting passive_income");
}

pub fn clear_passive_incomes(conn: &PgConnection, wallet_id: &str) {
    use crate::db::schema::passive_incomes::dsl::*;

    let _num_deleted = diesel::delete(passive_incomes.filter(wallet.eq(wallet_id)))
        .execute(conn)
        .expect("Error deleting passive_income");
}
