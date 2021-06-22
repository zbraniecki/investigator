use serde::{Deserialize, Serialize};

static TICKERS: &[(&str, &str)] = &[
    ("aapl", "Apple"),
];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PortfolioAssetInfo {
    pub ticker: String,
    pub name: String,
}

pub async fn fetch_stock_info(id: &str) -> Result<Vec<PortfolioAssetInfo>, ()> {
    assert_eq!(id, "top30stock");

    let result = TICKERS.iter().map(|(t, n)| {
        PortfolioAssetInfo {
            ticker: t.to_string(),
            name: n.to_string(),
        }
    }).collect();
    Ok(result)
}
