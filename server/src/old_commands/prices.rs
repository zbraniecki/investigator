use super::establish_connection;
use crate::api;
use crate::db;
use chrono::{NaiveDate, NaiveDateTime};

pub async fn fetch_asset_prices(args: &[String]) {
    let id = args.get(2).unwrap();
    let target = "usd";
    let asset_prices = api::fetch_asset_prices(&id, target).await.unwrap();

    let mut result: Vec<(NaiveDate, f64)> = vec![];

    let mut current_date = None;
    let mut values = vec![];
    for (ts, value) in &asset_prices.prices {
        // Something about 13 digits epoch vs 10 digits epoch and the
        // last 3 digits being microseconds? Not sure, but it works for now.
        let dt = NaiveDateTime::from_timestamp(*ts / 1000, 0);
        let new_date = dt.date();
        if let Some(date) = current_date {
            if date == new_date {
                values.push(value);
                continue;
            }
            let len = values.len();
            let sum: f64 = Iterator::sum(values.into_iter());
            let avg = sum / (len as f64);
            result.push((new_date, avg));
        }
        current_date = Some(new_date);
        values = vec![value];
    }
    let connection = establish_connection();
    db::clean_asset_prices(&connection, id);
    db::set_asset_prices(&connection, id, target, &result);
}
