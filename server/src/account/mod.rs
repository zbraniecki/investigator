pub mod portfolio;
pub mod target;

use actix_web::{web, Route};

pub fn get_views() -> Vec<(&'static str, Route)> {
    vec![
        ("/account/portfolio", web::get().to(portfolio::get_view)),
        ("/account/target", web::get().to(target::get_view)),
    ]
}
