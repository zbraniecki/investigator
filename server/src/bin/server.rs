// use crate::model;
use crate::db::establish_connection;
use actix_cors::Cors;
use actix_web::{web, HttpResponse, Route};
use actix_web::{App, HttpServer};
use investigator_server::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct State {
    //     pub coins: Arc<Mutex<Vec<model::Coin>>>,
//     pub prices: Arc<Mutex<model::PriceList>>,
//     pub wallets: Arc<Mutex<Vec<model::Wallet>>>,
}

impl State {
    pub async fn new() -> Self {
        // let coins = super::coins::get_data().await;
        // let prices = super::prices::get_data().await;
        // let wallets = super::wallets::get_data().await;
        Self {
            // coins: Arc::new(Mutex::new(coins)),
            // prices: Arc::new(Mutex::new(prices)),
            // wallets: Arc::new(Mutex::new(wallets)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Portfolio {
    name: String,
    assets: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Portfolios(Vec<Portfolio>);

#[derive(Deserialize)]
pub struct PriceViewQuery {
    #[serde(default)]
    refresh: bool,
}

pub async fn get_view(data: web::Data<State>, query: web::Query<PriceViewQuery>) -> HttpResponse {
    let connection = establish_connection();
    let portfolios = crate::portfolio::db::portfolio::filter(&connection);
    let result = portfolios
        .into_iter()
        .map(|port| {
            let assets = crate::portfolio::db::portfolio_assets::filter(&connection, port.id);

            let assets = assets.into_iter().map(|a| a.asset).collect();
            Portfolio {
                name: "crypto".to_string(),
                assets,
            }
        })
        .collect::<Vec<_>>();
    let mut r = vec![Portfolio {
        name: "S&P500".to_string(),
        assets: vec!["INTL".to_string(), "TSLA".to_string()],
    }];
    r.extend(result);
    let response = serde_json::to_string(&r).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

pub fn get_views() -> Vec<(&'static str, Route)> {
    vec![("/markets/portfolios", web::get().to(get_view))]
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = State::new().await;
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:1234")
            .allowed_origin("http://127.0.0.1:1234")
            .supports_credentials();
        let mut app = App::new().wrap(cors).data(state.clone());
        for (path, view) in get_views() {
            app = app.route(path, view)
        }
        app
    })
    .bind("127.0.0.1:8080")?
    .run();
    server.await
}
