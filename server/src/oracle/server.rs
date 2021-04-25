use crate::model;
use actix_web::{App, HttpServer};
use actix_cors::Cors;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct State {
    pub coins: Arc<Mutex<Vec<model::Coin>>>,
    pub prices: Arc<Mutex<model::PriceList>>,
}

impl State {
    pub async fn new() -> Self {
        let coins = super::coins::get_data().await;
        let prices = super::prices::get_data().await;
        Self {
            coins: Arc::new(Mutex::new(coins)),
            prices: Arc::new(Mutex::new(prices)),
        }
    }
}

pub async fn new_server() -> std::io::Result<()> {
    let state = State::new().await;
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:1234")
            .allowed_origin("http://127.0.0.1:1234")
            .supports_credentials();
        let mut app = App::new()
            .wrap(cors)
            .data(state.clone());
        for (path, view) in super::get_views() {
            app = app.route(path, view)
        }
        app
    })
    .bind("127.0.0.1:8080")?
        .run();
    server.await
}
