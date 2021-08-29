use crate::model;
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct State {
    pub watchlist: Arc<Mutex<Vec<model::WatchList>>>,
    pub portfolio: Arc<Mutex<Vec<model::Portfolio>>>,
    pub strategy: Arc<Mutex<Vec<model::Strategy>>>,
}

impl State {
    pub async fn new() -> Self {
        let watchlist = super::watchlist::get_data().await;
        let portfolio = super::portfolio::get_data().await;
        let strategy = super::strategy::get_data().await;
        Self {
            watchlist: Arc::new(Mutex::new(watchlist)),
            portfolio: Arc::new(Mutex::new(portfolio)),
            strategy: Arc::new(Mutex::new(strategy)),
        }
    }
}

pub async fn new_server() -> std::io::Result<Server> {
    let state = State::new().await;
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:1234")
            .allowed_origin("http://127.0.0.1:1234")
            .supports_credentials();
        let mut app = App::new().wrap(cors).data(state.clone());
        for (path, view) in super::get_views() {
            app = app.route(path, view)
        }
        app
    })
    .bind("127.0.0.1:8081")?
    .run();
    Ok(server)
}
