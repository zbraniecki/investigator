use super::models::AssetInfo as AssetPriceInfo;
use serde::{Deserialize, Serialize};

static INFO_URL: &str =
    "https://api.coingecko.com/api/v3/coins/{ID}?tickers=false&market_data=false";

static PRICE_INFO_URL: &str =
    "https://api.coingecko.com/api/v3/coins/markets?vs_currency={VS}&ids={IDS}&order=market_cap_desc&per_page=5&page=1&sparkline=false&price_change_percentage=1h%2C24h%2C7d%2C14d%2C30d%2C200d%2C1y";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssetInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

pub async fn fetch_info(id: &str) -> Result<AssetInfo, ()> {
    use actix_web::client::Client;
    let client = Client::default();

    let info_url = INFO_URL.replace("{ID}", id);

    let resp = client
        .get(info_url)
        .header("User-Agent", "Actix-web")
        .send()
        .await;
    if let Ok(mut resp) = resp {
        let body = resp.body().await.unwrap();

        let asset_info: AssetInfo = serde_json::from_slice(&body).unwrap();

        Ok(asset_info)
    } else {
        Err(())
    }
}

pub async fn fetch_price_info(ids: Vec<String>) -> Result<Vec<AssetPriceInfo>, ()> {
    use actix_web::client::Client;
    let client = Client::default();

    let info_url = PRICE_INFO_URL
        .replace("{VS}", "usd")
        .replace("{IDS}", &ids.join("%2C"));

    let resp = client
        .get(info_url)
        .header("User-Agent", "Actix-web")
        .send()
        .await;
    if let Ok(mut resp) = resp {
        let body = resp.body().await.unwrap();

        let asset_info: Vec<AssetPriceInfo> = serde_json::from_slice(&body).unwrap();

        Ok(asset_info)
    } else {
        Err(())
    }
}
