use serde::{Deserialize, Serialize};
use chrono::prelude::*;

// {
//   "id": "bitcoin",
//   "symbol": "btc",
//   "name": "Bitcoin",
//   "image": "https://assets.coingecko.com/coins/images/1/large/bitcoin.png?1547033579",
//   "current_price": 55489,
//   "market_cap": 1039411052850,
//   "market_cap_rank": 1,
//   "fully_diluted_valuation": 1168036950459,
//   "total_volume": 63005120513,
//   "high_24h": 57270,
//   "low_24h": 53820,
//   "price_change_24h": 1668.52,
//   "price_change_percentage_24h": 3.10018,
//   "market_cap_change_24h": 27002465431,
//   "market_cap_change_percentage_24h": 2.66715,
//   "circulating_supply": 18687450,
//   "total_supply": 21000000,
//   "max_supply": 21000000,
//   "ath": 64805,
//   "ath_change_percentage": -14.49002,
//   "ath_date": "2021-04-14T11:54:46.763Z",
//   "atl": 67.81,
//   "atl_change_percentage": 81621.46126,
//   "atl_date": "2013-07-06T00:00:00.000Z",
//   "roi": null,
//   "last_updated": "2021-04-21T07:36:34.511Z"
// },
#[derive(Serialize, Deserialize, Clone)]
pub struct Price {
    pub pair: (String, String),
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Holding {
    pub symbol: String,
    pub quantity: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Target {
    pub symbol: String,
    #[serde(default)]
    pub contains: Vec<String>,
    pub percent: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PriceList {
    // more meta - source etc.
    pub last_updated: DateTime<Utc>,
    #[serde(rename="price")]
    pub prices: Vec<Price>,
}
