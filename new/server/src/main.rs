#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod api;
pub mod db;
pub mod models;
pub mod schema;
pub mod commands;

use std::env;

#[actix_web::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    commands::handle_command(&args).await;
}
