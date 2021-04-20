use actix_web::{get, web, HttpResponse, Result, HttpServer, App};
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct Price {
    pub pair: (String, String),
    pub value: f64
}

struct ServerState {
    pub prices: Vec<Price>,
}

fn get_prices() -> Vec<Price> {
    #[derive(Serialize, Deserialize)]
    struct PriceList {
        price: Vec<Price>,
    }

    let source = fs::read_to_string("res/prices.toml")
        .expect("Something went wrong reading the file");
    let result: PriceList = toml::from_str(&source).unwrap();
    result.price
}

#[get("/")]
async fn index(data: web::Data<ServerState>) -> HttpResponse {
    let prices = &data.prices;
    HttpResponse::Ok()
       .content_type("application/json")
       .body(serde_json::to_string(prices).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let prices = get_prices();

    HttpServer::new(move || {
        App::new()
            .data(ServerState {
                prices: prices.clone(),

            })
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
