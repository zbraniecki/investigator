use crate::models::Price;
use crate::models::NewPrice;
use diesel::prelude::*;

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
