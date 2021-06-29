use serde::{Deserialize, Serialize};

static TOP_10_TICKERS: &[(&str, &str)] =
    &[("aapl", "Apple"), ("msft", "Microsoft"), ("amzn", "Amazon")];

static FIDELITY_TICKERS: &[(&str, &str)] = &[
    ("lev", "The Lion Electrict Company"),
    ("chpt", "Chargepoint"),
    ("tpgy", "TPGY"),
    ("prnt", "PRNT"),
    ("clii", "CLII"),
    ("arkq", "ARKQ"),
];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PortfolioAssetInfo {
    pub ticker: String,
    pub name: String,
}

pub async fn fetch_stock_info(id: &str) -> Result<Vec<PortfolioAssetInfo>, ()> {
    let tickers = match id {
        "top30stock" => TOP_10_TICKERS,
        "fidelity" => FIDELITY_TICKERS,
        _ => panic!("Unknown portfolio"),
    };

    let result = tickers
        .iter()
        .map(|(t, n)| PortfolioAssetInfo {
            ticker: t.to_string(),
            name: n.to_string(),
        })
        .collect();
    Ok(result)
}
