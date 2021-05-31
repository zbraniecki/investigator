use crate::api::CoinInfo;
use crate::models::Coin;
use crate::models::NewCoin;
use diesel::prelude::*;

pub fn create_coin(conn: &SqliteConnection, id: &str, symbol: Option<&str>, name: Option<&str>) {
    use crate::schema::coins;

    let new_coin = NewCoin { id, symbol, name };

    diesel::insert_or_ignore_into(coins::table)
        .values(&new_coin)
        .execute(conn)
        .expect("Error saving new post");
}

pub fn remove_coin(conn: &SqliteConnection, delete_id: &str) {
    use crate::schema::coins::dsl::*;

    let _num_deleted = diesel::delete(coins.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting coins");
}

pub fn get_coins(conn: &SqliteConnection) -> Vec<Coin> {
    use crate::schema::coins::dsl::*;

    let results = coins
        .order(id.desc())
        .load::<Coin>(conn)
        .expect("Error loading coins");
    results
}

pub fn set_coin_info(conn: &SqliteConnection, coin_id: &str, coin_info: &CoinInfo) {
    use crate::schema::coins::dsl::*;

    diesel::update(coins.find(coin_id))
        .set((symbol.eq(&coin_info.symbol), name.eq(&coin_info.name)))
        .execute(conn)
        .expect(&format!("Unable to find coin {}", coin_id));
}
