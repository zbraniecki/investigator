use crate::model::*;
use std::{
    fs,
    collections::HashMap,
    cell::RefCell,
};

pub struct SessionState {
    pub wallets: Vec<String>,
    pub exchanges: Vec<String>,
    pub transactions: Vec<Transaction>,
    pub portfolios: Vec<Portfolio>,
    pub allocations: Vec<Allocation>,
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

fn get_allocations() -> Vec<Allocation> {
    let source =
        fs::read_to_string("data/allocations.toml").expect("Something went wrong reading the file");
    let result: HashMap<String, HashMap<String, f64>> = toml::from_str(&source).unwrap();

    result.into_iter().map(|(key, value)| {
        Allocation {
            name: key,
            values: value
        }
    }).collect()
}

fn save_allocations(allocations: &[Allocation]) {
    use std::io::Write;
    let result: HashMap<String, HashMap<String, f64>> = allocations.iter().map(|alloc| {
        (alloc.name.clone(), alloc.values.clone())
    }).collect();
    let serialized = toml::to_string(&result).unwrap();
    let mut file = std::fs::File::create("data/allocations.toml").expect("create failed");
    file.write_all(serialized.as_bytes()).expect("write failed");
}

impl SessionState {
    pub fn new() -> Self {
        let transactions = get_transactions();
        let wallets = vec!["rcu", "coinbase.com", "coinbase.pro"];
        let exchanges = vec!["coinbase.com", "coinbase.pro", "blockfi"];
        let portfolios = get_portfolios();
        let allocations = get_allocations();

        Self {
            wallets: wallets.into_iter().map(Into::into).collect(),
            exchanges: exchanges.into_iter().map(Into::into).collect(),
            transactions: transactions.into_iter().map(Into::into).collect(),
            portfolios: portfolios.into_iter().map(Into::into).collect(),
            allocations,
        }
    }

    pub fn save(&self) {
        save_allocations(&self.allocations);
    }
}

pub struct Session {
    pub state: RefCell<SessionState>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(SessionState::new()),
        }
    }
}


