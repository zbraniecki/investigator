use super::establish_connection;
use crate::api;

pub async fn fetch_coin_prices(args: &[String]) {
    let id = args.get(2).unwrap();
    let coin_prices = api::fetch_coin_prices(&id).await.unwrap();
    println!("{:#?}", coin_prices);
    // let connection = establish_connection();
    // db::set_coin_info(&connection, id, &coin_info);
}
