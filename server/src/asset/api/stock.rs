use super::super::models::AssetInfo;
use chrono::{Duration, Utc};
use yahoo_finance::{history, Timestamped};

pub async fn fetch_info(id: &str) -> Result<(), ()> {
    panic!();
}

pub async fn fetch_price_info(ids: Vec<String>) -> Result<Vec<AssetInfo>, ()> {
    let now = Utc::now();
    let mut result = vec![];
    for id in ids {
        let data = history::retrieve_range(&id, now - Duration::days(1), None)
            .await
            .unwrap();
        let bar = data.get(0).unwrap();
        let date = bar.datetime().format("%b %e %Y");
        let value = bar.close;
        result.push(AssetInfo {
            asset: id.to_string(),
            reference_asset: "usd".to_string(),
            current_price: Some(value),
            market_cap: None,
            market_cap_rank: None,
            total_volume: None,
            high_24h: None,
            low_24h: None,
            price_change_24h: None,
            market_cap_change_24h: None,
            market_cap_change_percentage_24h: None,
            circulating_supply: None,
            total_supply: None,
            max_supply: None,
            ath: None,
            ath_change_percentage: None,
            ath_date: None,
            atl: None,
            atl_change_percentage: None,
            atl_date: None,
            last_updated: None,
            price_change_percentage_1h: None,
            price_change_percentage_24h: None,
            price_change_percentage_7d: None,
            price_change_percentage_14d: None,
            price_change_percentage_30d: None,
            price_change_percentage_200d: None,
            price_change_percentage_1y: None,
        });
    }
    Ok(result)
}
