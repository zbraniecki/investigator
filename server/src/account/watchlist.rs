use actix_web::{web, HttpResponse};
use super::server;
use crate::model;
use serde::{Serialize, Deserialize};
use std::fs;

pub async fn get_view(data: web::Data<server::State>) -> HttpResponse {
    let watchlist = data.watchlist.lock().unwrap();
    let response = serde_json::to_string(&*watchlist).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

pub async fn get_data() -> Vec<model::WatchList> {
    read_watchlist().await
}

async fn read_watchlist() -> Vec<model::WatchList> {
    #[derive(Serialize, Deserialize)]
    struct WatchList {
        watchlist: Vec<model::WatchList>,
    }

    let path = "res/account/watchlist.toml";

    if !fs::metadata(path).is_ok() {
        vec![]
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: WatchList = toml::from_str(&source).unwrap();
        result.watchlist
    }
}
