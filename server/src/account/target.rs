use actix_web::{web, HttpResponse};
use crate::ServerState;
use crate::model;
use serde::{Serialize, Deserialize};
use std::fs;

pub async fn get_view(data: web::Data<ServerState>) -> HttpResponse {
    let target = data.target.lock().unwrap();
    let response = serde_json::to_string(&*target).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

pub async fn get_data() -> Vec<model::Target> {
    read_target().await
}

async fn read_target() -> Vec<model::Target> {
    #[derive(Serialize, Deserialize)]
    struct TargetList {
        coin: Vec<model::Target>,
    }

    let path = "res/target.toml";

    if !fs::metadata(path).is_ok() {
        vec![]
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: TargetList = toml::from_str(&source).unwrap();
        result.coin
    }
}
