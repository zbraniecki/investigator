pub mod coins;
pub mod prices;
pub mod server;
pub mod wallets;

use actix_web::{web, Route};

pub fn get_views() -> Vec<(&'static str, Route)> {
    vec![
        ("/oracle/prices", web::get().to(prices::get_view)),
        ("/oracle/coins", web::get().to(coins::get_view)),
        ("/oracle/wallets", web::get().to(wallets::get_view)),
    ]
}
