use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

static PRICES_URL: &str =
    "https://api.coingecko.com/api/v3/coins/{ID}/market_chart/range?vs_currency={CURRENCY}&from={FROM}&to={TO}";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CoinPrices {
    pub prices: Vec<(i64, f64)>,
}

pub async fn fetch_coin_prices(id: &str, target: &str) -> Result<CoinPrices, ()> {
    let dt_end: DateTime<Local> = Local::now();
    let dt_start: DateTime<Local> = dt_end - Duration::days(3);
    let ts_end = dt_end.timestamp();
    let ts_start = dt_start.timestamp();
    use actix_web::client::Client;
    let client = Client::default();

    let prices_url = PRICES_URL
        .replace("{ID}", id)
        .replace("{CURRENCY}", target)
        .replace("{FROM}", &ts_start.to_string())
        .replace("{TO}", &ts_end.to_string());

    let resp = client
        .get(prices_url)
        .header("User-Agent", "Actix-web")
        .send()
        .await;
    if let Ok(mut resp) = resp {
        let body = resp.body().await.unwrap();

        let coin_prices: CoinPrices = serde_json::from_slice(&body).unwrap();

        Ok(coin_prices)
    } else {
        Err(())
    }
}
