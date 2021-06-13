use investigator_server::*;

use std::env;

#[actix_web::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    commands::handle_command(&args).await;
}
