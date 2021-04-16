use crate::model::*;
use std::fs;

pub struct State {
    pub wallets: Vec<String>,
    pub exchanges: Vec<String>,
    pub transactions: Vec<Transaction>,
    pub portfolios: Vec<Portfolio>,
}

fn get_transactions() -> Vec<Transaction> {
    let json = fs::read_to_string("data/transactions.json")
        .expect("Something went wrong reading the file");
    let mut result: Vec<Transaction> = serde_json::from_str(&json).unwrap();
    result.sort_by(|a, b| a.ts.cmp(&b.ts).reverse());
    result
}

fn get_portfolios() -> Vec<Portfolio> {
    let json =
        fs::read_to_string("data/portfolios.json").expect("Something went wrong reading the file");
    let result: Vec<Portfolio> = serde_json::from_str(&json).unwrap();
    result
}

impl State {
    pub fn new() -> Self {
        let transactions = get_transactions();
        let wallets = vec!["rcu", "coinbase.com", "coinbase.pro"];
        let exchanges = vec!["coinbase.com", "coinbase.pro", "blockfi"];
        let portfolios = get_portfolios();

        Self {
            wallets: wallets.into_iter().map(Into::into).collect(),
            exchanges: exchanges.into_iter().map(Into::into).collect(),
            transactions: transactions.into_iter().map(Into::into).collect(),
            portfolios: portfolios.into_iter().map(Into::into).collect(),
        }
    }
}
