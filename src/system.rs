use crate::model::*;

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
        ];
        let exchanges = vec![
            (
                "coinbase.com",
                "Coinbase.com",
                Some("CB"),
                "https://www.coinbase.com",
            ),
            ("blockfi", "BlockFi", None, "https://www.blockfi.com"),
        ];
        Self {
            wallets: wallets.into_iter().map(Into::into).collect(),
            exchanges: exchanges.into_iter().map(Into::into).collect(),
        }
    }

    pub fn get_wallet(&self, id: &str) -> Option<&Wallet> {
        self.wallets.iter().find(|wallet| wallet.id == id)
    }

    pub fn get_exchange(&self, id: &str) -> Option<&Exchange> {
        self.exchanges.iter().find(|exchange| exchange.id == id)
    }
}
