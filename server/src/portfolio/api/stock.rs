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

static EJ1_TICKERS: &[(&str, &str)] = &[
    ("amzn", "Amazon"),
    ("bam", "Brookfield Asset Mgmt Inc"),
    ("bamr", "Brookfield Asset Mgmt Reins A"),
    ("etn", "Eaton Corportaion Plc"),
    ("nflx", "Netflix Inc"),
    ("pg", "Procter & Gamble Co"),
    ("tsla", "Tesla Inc"),
    ("vz", "Verizon Communications"),
];

static EJ2_TICKERS: &[(&str, &str)] = &[
    ("atvi", "Activision Blizzard Inc"),
    ("all", "AllState Corp"),
    ("beam", "Beam Therapeutics Inc"),
    ("c", "Citigroup Inc"),
    ("dlr", "Digital Realty Trust Inc"),
    ("edit", "Editas Medicine Inc"),
    ("emr", "Eerson Electric Co"),
    ("fmc", "CMD_CORP"),
    ("gd", "General Dynamics Corp"),
    ("ibm", "IBM"),
    ("lmt", "Locheed Martin Corp"),
    ("nee", "Nextera Energy Inc"),
    ("pg", "Procter & Gamble Co"),
    ("regn", "Regeneron Pharmaceuticals Inc"),
    ("spwr", "SunPower Corp"),
    ("tm", "Toyota Motor Corp New"),
    ("vrtx", "Vertex Pharmaceuticals Inc"),
    ("dis", "Walt Disney Co"),
    ("zts", "Zoetis Inc"),
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
        "ej1" => EJ1_TICKERS,
        "ej2" => EJ2_TICKERS,
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
