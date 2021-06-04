use serde::{Deserialize, Serialize};

static INFO_URL: &str =
    "https://api.coingecko.com/api/v3/coins/{ID}?tickers=false&market_data=false";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssetInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

pub async fn fetch_asset_info(id: &str) -> Result<AssetInfo, ()> {
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
