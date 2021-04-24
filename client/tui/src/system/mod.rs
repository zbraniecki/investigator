use crate::model::*;
use serde::{Deserialize, Serialize};
use std::fs;

pub struct System {
    pub data: SystemData,
}

impl System {
    pub fn new() -> Self {
        Self {
            data: SystemData::new(),
        }
    }
}

pub struct SystemData {
    pub exchanges: Vec<Exchange>,
    pub wallets: Vec<Wallet>,
    pub prices: Vec<Price>,
    pub tokens: Vec<Token>,
}

async fn get_prices() -> Vec<Price> {
    use hyper::body::HttpBody;
    use tokio::io::stdout;
    use tokio::io::AsyncWriteExt;
    use hyper::Client;

    let client = Client::new();

    let uri = "http://127.0.0.1:8080".parse().unwrap();

    let mut s = String::new();
    let resp = client.get(uri).await.unwrap();

    let res = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    
    serde_json::from_slice(&res).unwrap()
}

fn get_tokens() -> Vec<Token> {
    #[derive(Serialize, Deserialize)]
    struct CurrencyList {
        currency: Vec<Token>,
    }
    let source =
        fs::read_to_string("data/tokens.toml").expect("Something went wrong reading the file");
    let result: CurrencyList = toml::from_str(&source).unwrap();
    result.currency
}

impl SystemData {
    pub fn new() -> Self {
        let wallets = vec![
            (
                "rcu",
                "Redwood CU",
                Some("RCU"),
                "https://online.redwoodcu.org",
            ),
            (
                "coinbase.com",
                "Coinbase.com",
                Some("CB"),
                "https://www.coinbase.com",
            ),
            (
                "coinbase.pro",
                "Coinbase.pro",
                Some("CBPro"),
                "https://pro.coinbase.com",
            ),
        ];
        let exchanges = vec![
            (
                "coinbase.com",
                "Coinbase.com",
                Some("CB"),
                "https://www.coinbase.com",
            ),
            (
                "coinbase.pro",
                "Coinbase.pro",
                Some("CBPro"),
                "https://pro.coinbase.com",
            ),
            ("blockfi", "BlockFi", None, "https://www.blockfi.com"),
        ];
        let prices = get_prices();
        Self {
            wallets: wallets.into_iter().map(Into::into).collect(),
            exchanges: exchanges.into_iter().map(Into::into).collect(),
            prices: vec![],
            // prices: prices.into_iter().map(Into::into).collect(),
            tokens: get_tokens(),
        }
    }

    pub fn get_wallet(&self, id: &str) -> Option<&Wallet> {
        self.wallets.iter().find(|wallet| wallet.id == id)
    }

    pub fn get_exchange(&self, id: &str) -> Option<&Exchange> {
        self.exchanges.iter().find(|exchange| exchange.id == id)
    }

    pub fn get_token(&self, symbol: &str) -> Option<&Token> {
        self.tokens.iter().find(|token| token.symbol == symbol)
    }

    pub fn get_price(&self, from: &str, to: &str) -> Option<f64> {
        self.prices
            .iter()
            .find(|price| price.pair.0 == from && price.pair.1 == to)
            .map(|price| price.value)
    }
}
