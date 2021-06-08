#[macro_use]
extern crate diesel;
extern crate dotenv;

mod asset;
mod identity;
mod price;
mod service;

mod commands;
pub mod db;

use std::env;

#[actix_web::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    commands::handle_command(&args).await;
}
