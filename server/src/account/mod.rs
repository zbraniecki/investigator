pub mod server;
pub mod portfolio;
pub mod strategy;
pub mod watchlist;

use actix_web::{web, Route};

pub fn get_views() -> Vec<(&'static str, Route)> {
    vec![
        ("/account/watchlist", web::get().to(watchlist::get_view)),
        ("/account/portfolio", web::get().to(portfolio::get_view)),
        ("/account/strategy", web::get().to(strategy::get_view)),
    ]
}
