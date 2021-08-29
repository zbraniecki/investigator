use super::server;
use crate::model;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fs;

pub async fn get_view(data: web::Data<server::State>) -> HttpResponse {
    let coins = data.wallets.lock().unwrap();
    let response = serde_json::to_string(&*coins).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

pub async fn get_data() -> Vec<model::Wallet> {
    read_wallets().await
}

pub async fn read_wallets() -> Vec<model::Wallet> {
    #[derive(Serialize, Deserialize)]
    struct WalletList {
        wallet: Vec<model::Wallet>,
    }

    let path = "res/oracle/wallets.toml";

    if !fs::metadata(path).is_ok() {
        vec![]
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: WalletList = toml::from_str(&source).unwrap();
        result.wallet
    }
}
