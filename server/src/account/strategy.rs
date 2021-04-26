use actix_web::{web, HttpResponse};
use super::server;
use crate::model;
use serde::{Serialize, Deserialize};
use std::fs;

pub async fn get_view(data: web::Data<server::State>) -> HttpResponse {
    let strategy = data.strategy.lock().unwrap();
    let response = serde_json::to_string(&*strategy).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

pub async fn get_data() -> Vec<model::Strategy> {
    read_strategy().await
}

async fn read_strategy() -> Vec<model::Strategy> {
    #[derive(Serialize, Deserialize)]
    struct StrategyList {
        coin: Vec<model::Target>,
    }

    let path = "res/account/strategy.toml";

    if !fs::metadata(path).is_ok() {
        vec![]
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: StrategyList = toml::from_str(&source).unwrap();
        vec![
            model::Strategy {
                id: "crypto".to_string(),
                name: "Crypto".to_string(),
                targets: result.coin
            }
        ]
    }
}
