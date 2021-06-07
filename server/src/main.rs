#[macro_use]
extern crate diesel;
extern crate dotenv;

mod asset;
mod identity;

// pub mod api;
mod commands;
pub mod db;
// pub mod models;

use std::env;

#[actix_web::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    commands::handle_command(&args).await;
}
