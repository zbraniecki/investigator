use crate::models::NewPrice;
use crate::models::Price;
use chrono::NaiveDate;
use diesel::prelude::*;

pub fn clean_coin_prices<'a>(conn: &SqliteConnection, coin_id: &'a str) {
    use crate::schema::prices::dsl::*;

    let _num_deleted = diesel::delete(prices.filter(base.eq(coin_id)))
        .execute(conn)
        .expect("Error deleting prices");
}

pub fn set_coin_prices(
    conn: &SqliteConnection,
    base: &str,
    target: &str,
    prices: &[(NaiveDate, f64)],
) {
    use crate::schema::prices;

    for price in prices {
        let new_price = NewPrice {
            base,
            target,
            ts: price.0.and_hms(0, 0, 0),
            value: price.1,
        };

        diesel::insert_or_ignore_into(prices::table)
            .values(&new_price)
            .execute(conn)
            .expect("Error saving new post");
    }
}

pub fn fetch_prices(conn: &SqliteConnection, base_id: &str, target_id: &str) -> Option<Vec<Price>> {
    use crate::schema::prices::dsl::*;

    let results = prices
        .filter(base.eq(base_id))
        .filter(target.eq(target_id))
        .order(ts.desc())
        .load::<Price>(conn)
        .expect("Error loading prices");
    Some(results)
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
