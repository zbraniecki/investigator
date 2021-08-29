use super::server;
use crate::model;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fs;

pub async fn get_view(data: web::Data<server::State>) -> HttpResponse {
    let portfolio = data.portfolio.lock().unwrap();
    let response = serde_json::to_string(&*portfolio).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

pub async fn get_data() -> Vec<model::Portfolio> {
    read_portfolio().await
}

async fn read_portfolio() -> Vec<model::Portfolio> {
    #[derive(Serialize, Deserialize)]
    struct HoldingList {
        holding: Vec<model::Holding>,
    }

    let path = "res/account/portfolio.toml";

    if !fs::metadata(path).is_ok() {
        vec![]
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: HoldingList = toml::from_str(&source).unwrap();
        vec![model::Portfolio {
            id: "crypto".to_string(),
            name: "Crypto".to_string(),
            holdings: result.holding,
        }]
    }
}
