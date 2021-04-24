mod oracle;
mod account;
mod model;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use std::sync::{Arc, Mutex};
use futures::future;

#[derive(Clone)]
pub struct ServerState {
    pub coins: Arc<Mutex<Vec<model::Coin>>>,
    pub prices: Arc<Mutex<model::PriceList>>,
    pub portfolio: Arc<Mutex<Vec<model::Holding>>>,
    pub target: Arc<Mutex<Vec<model::Target>>>,
}

impl ServerState {
    pub async fn new() -> Self {
        let coins = oracle::coins::get_data().await;
        let prices = oracle::prices::get_data().await;
        let portfolio = account::portfolio::get_data().await;
        let target = account::target::get_data().await;
        Self {
            prices: Arc::new(Mutex::new(prices)),
            coins: Arc::new(Mutex::new(coins)),
            portfolio: Arc::new(Mutex::new(portfolio)),
            target: Arc::new(Mutex::new(target)),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = ServerState::new().await;

    let oracle_state = state.clone();
    let oracle_server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:1234")
            .allowed_origin("http://127.0.0.1:1234")
            .supports_credentials();
        let mut app = App::new()
            .wrap(cors)
            .data(oracle_state.clone());
        for (path, view) in oracle::get_views() {
            app = app.route(path, view)
        }
        app
    })
    .bind("127.0.0.1:8080")?
    .run();

    let account_state = state.clone();
    let account_server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:1234")
            .allowed_origin("http://127.0.0.1:1234")
            .supports_credentials();
        let mut app = App::new()
            .wrap(cors)
            .data(account_state.clone());
        for (path, view) in account::get_views() {
            app = app.route(path, view)
        }
        app
    })
    .bind("127.0.0.1:8081")?
    .run();

    future::try_join(oracle_server, account_server).await?;

    Ok(())
}
