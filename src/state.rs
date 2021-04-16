use crate::model::*;

pub struct State {
    pub wallets: Vec<String>,
    pub exchanges: Vec<String>,
    pub transactions: Vec<Transaction>,
    pub portfolios: Vec<Portfolio>,
}

impl State {
    pub fn new() -> Self {
        let wallets = vec!["rcu", "coinbase.com"];
        let exchanges = vec!["coinbase.com", "blockfi"];
        let transactions = vec![(
            "rcu",
            "coinbase.com",
            Value::from((100, "USD")),
            Value::from((100, "USD")),
            None,
            None,
            1001,
        )];
        let portfolios = vec![("Main Crypto", None)];

        Self {
            wallets: wallets.into_iter().map(Into::into).collect(),
            exchanges: exchanges.into_iter().map(Into::into).collect(),
            transactions: transactions.into_iter().map(Into::into).collect(),
            portfolios: portfolios.into_iter().map(Into::into).collect(),
        }
    }
}
