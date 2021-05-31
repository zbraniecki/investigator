use crate::api::CoinInfo;
use crate::models::{Coin, Price};
use crate::models::{NewCoin, NewPrice};
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

pub fn set_coin_price<'a>(conn: &SqliteConnection, base: &'a str, target: &'a str, value: f64) {
    use crate::schema::prices;
    use chrono::{DateTime, Local, NaiveDateTime};

    let now: DateTime<Local> = Local::now();
    let ts: NaiveDateTime = NaiveDateTime::from_timestamp(now.timestamp(), 0);

    let new_price = NewPrice {
        base,
        target,
        ts,
        value,
    };

    diesel::insert_or_ignore_into(prices::table)
        .values(&new_price)
        .execute(conn)
        .expect("Error saving new post");
}

pub fn get_current_price<'a>(
    conn: &SqliteConnection,
    base_query: &'a str,
    target_query: &'a str,
) -> Option<Price> {
    use crate::schema::prices::dsl::*;
    use chrono::{DateTime, Local, NaiveDateTime};

    let now: DateTime<Local> = Local::now();
    let timestamp: NaiveDateTime = NaiveDateTime::from_timestamp(now.timestamp(), 0);

    if base_query == target_query {
        return Some(Price {
            base: base_query.to_string(),
            target: target_query.to_string(),
            ts: timestamp,
            value: 1.0,
        });
    }

    let results = prices
        .filter(base.eq(base_query))
        .filter(target.eq(target_query))
        .order(ts.desc())
        .limit(1)
        .load::<Price>(conn)
        .expect("Error loading prices");
    results.get(0).cloned()
}
