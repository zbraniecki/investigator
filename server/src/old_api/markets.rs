use serde::{Deserialize, Serialize};

static MARKETS_URL: &str =
    "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=10&page=1&sparkline=false";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketAssetInfo {
    pub id: String,
}

pub async fn fetch_market_assets(id: &str) -> Result<Vec<MarketAssetInfo>, ()> {
    use actix_web::client::Client;
    let client = Client::default();

    let markets_url = MARKETS_URL.replace("{ID}", id);

    let resp = client
        .get(markets_url)
        .header("User-Agent", "Actix-web")
        .send()
        .await;
    if let Ok(mut resp) = resp {
        let body = resp.body().await.unwrap();

        let market_assets: Vec<MarketAssetInfo> = serde_json::from_slice(&body).unwrap();

        Ok(market_assets)
    } else {
        Err(())
    }
}
