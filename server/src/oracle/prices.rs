use super::server;
use crate::model;
use actix_web::{web, HttpResponse};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

static PRICE_URL: &str = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids={IDS}&order=market_cap_desc&per_page=100&page=1&sparkline=false";

#[derive(Deserialize)]
pub struct PriceViewQuery {
    #[serde(default)]
    refresh: bool,
}

pub async fn get_view(
    data: web::Data<server::State>,
    query: web::Query<PriceViewQuery>,
) -> HttpResponse {
    let prices = if query.refresh {
        if let Ok(mut prices) = data.prices.lock() {
            update_data(&mut prices).await;
            Some(prices)
        } else {
            None
        }
    } else {
        if let Ok(prices) = data.prices.lock() {
            Some(prices)
        } else {
            None
        }
    };
    let response = if let Some(prices) = prices {
        serde_json::to_string(&*prices).unwrap()
    } else {
        String::from("")
    };
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
    #[derive(Serialize, Deserialize, Clone, Debug)]
    struct ExternalPrice {
        symbol: String,
        id: String,
        current_price: Option<f64>,
        market_cap: Option<f64>,
        price_change_percentage_24h: Option<f64>,
        market_cap_change_percentage_24h: Option<f64>,
    }
    use actix_web::client::Client;
    let client = Client::default();

    let coin_str: Vec<_> = coins.iter().map(|coin| coin.id.to_string()).collect();
    // println!("{:#?}", coin_str);

    let ids = coin_str.join("%2C");
    let price_url = PRICE_URL.replace("{IDS}", &ids);

    let resp = client
        .get(price_url)
        .header("User-Agent", "Actix-web")
        .send()
        .await;
    if let Ok(mut resp) = resp {
        let body = resp.body().await.unwrap();

        let mut prices: Vec<ExternalPrice> = serde_json::from_slice(&body).unwrap();

        let missing_prices: Vec<_> = coin_str
            .iter()
            .filter(|c| {
                prices
                    .iter()
                    .find(|r| r.id.as_str() == c.as_str())
                    .is_none()
            })
            .cloned()
            .collect();

        if !missing_prices.is_empty() {
            // println!("Missing ones: {:#?}", missing_prices);
            let ids = missing_prices.join("%2C");
            let price_url = PRICE_URL.replace("{IDS}", &ids);
            let resp = client
                .get(price_url)
                .header("User-Agent", "Actix-web")
                .send()
                .await;
            if let Ok(mut resp) = resp {
                let body = resp.body().await.unwrap();

                let prices2: Vec<ExternalPrice> = serde_json::from_slice(&body).unwrap();
                // println!("{:#?}", prices2);
                prices.extend(prices2);
            }
        }

        let results = prices
            .into_iter()
            .map(|price| model::Price {
                pair: (price.symbol, "USD".to_string()),
                value: price.current_price.unwrap_or(0.0),
                market_cap: price.market_cap.unwrap_or(0.0),
                price_change_percentage_24h: price.price_change_percentage_24h.unwrap_or(0.0),
                market_cap_change_percentage_24h: price.market_cap_change_percentage_24h.unwrap_or(0.0),
            })
            .collect();

        Ok(results)
    } else {
        Err(())
    }
}

async fn read_prices(coins: &[model::Coin]) -> model::PriceList {
    let path = "res/oracle/prices.toml";

    if !fs::metadata(path).is_ok() {
        let prices = fetch_prices(coins).await.unwrap_or_default();
        let last_updated: DateTime<Utc> = Utc::now();
        let price_list = model::PriceList {
            prices,
            last_updated,
        };
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
                    CoinID::Symbol(c) => c == &coin.id,
                    CoinID::SymbolAndID((_, id)) => id == &coin.id,
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
