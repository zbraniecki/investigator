use actix_web::{web, HttpResponse};
use super::server;
use crate::model;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;

static PRICE_URL: &str = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids={IDS}&order=market_cap_desc&per_page=100&page=1&sparkline=false";

#[derive(Deserialize)]
pub struct PriceViewQuery {
    #[serde(default)]
    refresh: bool
}

pub async fn get_view(data: web::Data<server::State>, query: web::Query<PriceViewQuery>) -> HttpResponse {
    let prices = if query.refresh {
        let mut prices = data.prices.lock().unwrap();
        update_data(&mut prices).await;
        prices
    } else {
        data.prices.lock().unwrap()
    };
    let response = serde_json::to_string(&*prices).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

pub async fn update_data(price_list: &mut model::PriceList) {
    let last_updated: DateTime<Utc> = Utc::now();
    let coins = get_supported_coins().await;
    if let Ok(prices) = fetch_prices(&coins).await {
        price_list.prices = prices;
        price_list.last_updated = last_updated;
    }
}

pub async fn get_data() -> model::PriceList {
    let coins = get_supported_coins().await;
    read_prices(&coins).await
}

async fn fetch_prices(coins: &[model::Coin]) -> Result<Vec<model::Price>, ()> {
    #[derive(Serialize, Deserialize, Clone)]
    struct ExternalPrice {
        symbol: String,
        current_price: f64,
        market_cap: f64,
        price_change_percentage_24h: f64,
        market_cap_change_percentage_24h: f64,
    }
    use actix_web::client::Client;
    let client = Client::default();

    let ids = coins
        .iter()
        .map(|coin| coin.id.to_string())
        .collect::<Vec<_>>()
        .join("%2C");
    let price_url = PRICE_URL.replace("{IDS}", &ids);

    let resp = client
        .get(price_url)
        .header("User-Agent", "Actix-web")
        .send()
        .await;
    if let Ok(mut resp) = resp {
        let body = resp.body().await.unwrap();

        let prices: Vec<ExternalPrice> = serde_json::from_slice(&body).unwrap();

        Ok(prices
            .into_iter()
            .map(|price| model::Price {
                pair: (price.symbol, "USD".to_string()),
                value: price.current_price,
                market_cap: price.market_cap,
                price_change_percentage_24h: price.price_change_percentage_24h,
                market_cap_change_percentage_24h: price.market_cap_change_percentage_24h,
            })
            .collect())
    } else {
        Err(())
    }

}

async fn read_prices(coins: &[model::Coin]) -> model::PriceList {
    let path = "res/oracle/prices.toml";

    if !fs::metadata(path).is_ok() {
        let prices = fetch_prices(coins).await.unwrap_or_default();
        let last_updated: DateTime<Utc> = Utc::now();
        let price_list = model::PriceList { prices, last_updated };
        let toml_string = toml::to_string(&price_list).unwrap();
        fs::write(path, toml_string).expect("Could not write to file!");
        price_list
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: model::PriceList = toml::from_str(&source).unwrap();
        result
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum CoinID {
    Symbol(String),
    SymbolAndID((String, String)),
}

async fn get_supported_coins() -> Vec<model::Coin> {
    let all_coins = super::coins::read_coins().await;
    let supported_coins = read_supported_coins();

    all_coins
        .into_iter()
        .filter(|coin| {
            supported_coins
                .iter()
                .find(|c| match c {
                    CoinID::Symbol(c) => c == &coin.symbol,
                    CoinID::SymbolAndID((s, id)) => s == &coin.symbol && id == &coin.id,
                })
                .is_some()
        })
        .collect()
}


fn read_supported_coins() -> Vec<CoinID> {
    #[derive(Serialize, Deserialize)]
    struct CoinList {
        coins: Vec<CoinID>,
    }

    let path = "res/oracle/supported_coins.toml";

    if !fs::metadata(path).is_ok() {
        vec![]
    } else {
        let source = fs::read_to_string(path).expect("Something went wrong reading the file");
        let result: CoinList = toml::from_str(&source).unwrap();
        result.coins
    }
}

