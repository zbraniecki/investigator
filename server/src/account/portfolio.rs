use actix_web::{web, HttpResponse};
use crate::ServerState;
use crate::model;
use serde::{Serialize, Deserialize};
use std::fs;

pub async fn get_view(data: web::Data<ServerState>) -> HttpResponse {
    let portfolio = data.portfolio.lock().unwrap();
    let response = serde_json::to_string(&*portfolio).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

pub async fn get_data() -> Vec<model::Holding> {
    read_portfolio().await
}

async fn read_portfolio() -> Vec<model::Holding> {
    #[derive(Serialize, Deserialize)]
    struct HoldingList {
        holding: Vec<model::Holding>,
    }

    let path = "res/portfolio.toml";

    if !fs::metadata(path).is_ok() {
        vec![]
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: HoldingList = toml::from_str(&source).unwrap();
        result.holding
    }
}
