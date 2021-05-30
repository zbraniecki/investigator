use serde::{Serialize, Deserialize};

static INFO_URL: &str = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids={IDS}&order=market_cap_desc&per_page=100&page=1&sparkline=false";

#[derive(Serialize, Deserialize, Clone)]
pub struct CoinInfo {
    id: String,
    symbol: String,
    name: String,
}

pub async fn fetch_coin_info(id: &str) -> Result<CoinInfo, ()> {
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

        let coin_info: CoinInfo = serde_json::from_slice(&body).unwrap();

        Ok(coin_info)
    } else {
        Err(())
    }
}
