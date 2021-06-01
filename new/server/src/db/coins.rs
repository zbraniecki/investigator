use crate::api::CoinInfo;
use crate::models::Coin;
use crate::models::NewCoin;
use diesel::prelude::*;

pub fn create_coin(conn: &PgConnection, id: &str, symbol: Option<&str>, name: Option<&str>) {
    use crate::db::schema::coins;

    let new_coin = NewCoin { id, symbol, name };

    diesel::insert_into(coins::table)
        .values(&new_coin)
        .execute(conn)
        .expect("Error saving new post");
}

pub fn remove_coin(conn: &PgConnection, delete_id: &str) {
    use crate::db::schema::coins::dsl::*;

    let _num_deleted = diesel::delete(coins.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting coins");
}

pub fn get_coins(conn: &PgConnection) -> Vec<Coin> {
    use crate::db::schema::coins::dsl::*;

    let results = coins
        .order(id.desc())
        .load::<Coin>(conn)
        .expect("Error loading coins");
    results
}

pub fn set_coin_info(conn: &PgConnection, coin_id: &str, coin_info: &CoinInfo) {
    use crate::db::schema::coins::dsl::*;

    diesel::update(coins.find(coin_id))
        .set((symbol.eq(&coin_info.symbol), name.eq(&coin_info.name)))
        .execute(conn)
        .expect(&format!("Unable to find coin {}", coin_id));
}
