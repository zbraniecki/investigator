use actix_web::{web, HttpResponse};
use super::server;
use crate::model;
use std::fs;
use serde::{Deserialize, Serialize};

static COIN_LIST_URL: &str = "https://api.coingecko.com/api/v3/coins/list?include_platform=false";

pub async fn get_view(data: web::Data<server::State>) -> HttpResponse {
    let coins = data.coins.lock().unwrap();
    let response = serde_json::to_string(&*coins).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

pub async fn get_data() -> Vec<model::Coin> {
    read_coins().await
}

async fn fetch_coins() -> Vec<model::Coin> {
    use actix_web::client::Client;
    let client = Client::default();

    let mut resp = client
        .get(COIN_LIST_URL) // <--- notice the "s" in "https://..."
        .header("User-Agent", "Actix-web")
        .send()
        .await
        .unwrap();

    let body = resp.body().limit(51200000).await.unwrap();

    serde_json::from_slice(&body).unwrap()
}

pub async fn read_coins() -> Vec<model::Coin> {
    #[derive(Serialize, Deserialize)]
    struct CoinList {
        coin: Vec<model::Coin>,
    }

    let path = "res/oracle/coins.toml";

    if !fs::metadata(path).is_ok() {
        let coins = fetch_coins().await;
        // let coins = vec![];
        let coin_list = CoinList { coin: coins };
        let toml_string = toml::to_string(&coin_list).unwrap();
        fs::write(path, toml_string).expect("Could not write to file!");
        coin_list.coin
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: CoinList = toml::from_str(&source).unwrap();
        result.coin
    }
}
