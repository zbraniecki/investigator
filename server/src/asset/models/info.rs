use crate::db::schema::assets_info;
use chrono;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Clone, Debug, Serialize, Deserialize)]
#[table_name = "assets_info"]
pub struct AssetInfo {
    #[serde(rename = "id")]
    pub asset: String,
    #[serde(default)]
    pub reference_asset: String,
    pub current_price: Option<f64>,
    pub market_cap: Option<i64>,
    pub market_cap_rank: Option<i64>,
    pub total_volume: Option<i64>,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub market_cap_change_24h: Option<f64>,
    pub market_cap_change_percentage_24h: Option<f64>,
    pub circulating_supply: Option<f64>,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub ath: Option<f64>,
    pub ath_change_percentage: Option<f64>,
    pub ath_date: Option<DateTime<Utc>>,
    pub atl: Option<f64>,
    pub atl_change_percentage: Option<f64>,
    pub atl_date: Option<DateTime<Utc>>,
    pub last_updated: Option<DateTime<Utc>>,
    #[serde(rename = "price_change_percentage_1h_in_currency")]
    pub price_change_percentage_1h: Option<f64>,
    #[serde(rename = "price_change_percentage_24h_in_currency")]
    pub price_change_percentage_24h: Option<f64>,
    #[serde(rename = "price_change_percentage_7d_in_currency")]
    pub price_change_percentage_7d: Option<f64>,
    #[serde(rename = "price_change_percentage_14d_in_currency")]
    pub price_change_percentage_14d: Option<f64>,
    #[serde(rename = "price_change_percentage_30d_in_currency")]
    pub price_change_percentage_30d: Option<f64>,
    #[serde(rename = "price_change_percentage_200d_in_currency")]
    pub price_change_percentage_200d: Option<f64>,
    #[serde(rename = "price_change_percentage_1y_in_currency")]
    pub price_change_percentage_1y: Option<f64>,
}
