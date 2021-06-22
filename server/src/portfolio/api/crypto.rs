use serde::{Deserialize, Serialize};

static COUNT: usize = 30;
static INFO_URL: &str =
    "https://api.coingecko.com/api/v3/coins/markets?vs_currency=USD&order=market_cap_desc&per_page={COUNT}&page=1&sparkline=false";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PortfolioAssetInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

pub async fn fetch_crypto_info(id: &str) -> Result<Vec<PortfolioAssetInfo>, ()> {
    assert_eq!(id, "top30crypto");
    use actix_web::client::Client;
    let client = Client::default();

    let info_url = INFO_URL.replace("{COUNT}", &COUNT.to_string());

    let resp = client
        .get(info_url)
        .header("User-Agent", "Actix-web")
        .send()
        .await;
    if let Ok(mut resp) = resp {
        let body = resp.body().await.unwrap();

        let asset_info: Vec<PortfolioAssetInfo> = serde_json::from_slice(&body).unwrap();

        Ok(asset_info)
    } else {
        Err(())
    }
}
