pub mod prices;
pub mod coins;
use actix_web::{web, Route};

pub fn get_views() -> Vec<(&'static str, Route)> {
    vec![
        ("/oracle/prices", web::get().to(prices::get_view)),
        ("/oracle/coins", web::get().to(coins::get_view)),
    ]
}
