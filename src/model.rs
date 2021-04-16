use crate::system::SystemData;
use chrono::prelude::*;
use float_pretty_print::PrettyPrintFloat;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Currency {
    pub symbol: String,
}

impl From<&str> for Currency {
    fn from(input: &str) -> Self {
        Self {
            symbol: input.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Value {
    pub quantity: f64,
    pub currency_symbol: String,
}

impl Value {
    pub fn format_with_precision(&self, short: bool, prec: usize) -> String {
        if short && self.currency_symbol == "USD" {
            format!("${:.1$}", self.quantity, prec)
        } else {
            format!("{} {:.2$}", self.quantity, self.currency_symbol, prec)
        }
    }

    pub fn format(&self, short: bool) -> String {
        let formatted = PrettyPrintFloat(self.quantity);
        if short && self.currency_symbol == "USD" {
            format!("${}", formatted)
        } else {
            format!("{} {}", formatted, self.currency_symbol)
        }
    }

    pub fn value_in(&self, symbol: &str, system_data: &SystemData) -> Value {
        if self.currency_symbol == symbol {
            self.clone()
        } else {
            let price = system_data
                .get_price(&self.currency_symbol, symbol)
                .unwrap();
            Value {
                currency_symbol: symbol.to_string(),
                quantity: self.quantity * price,
            }
        }
    }

    pub fn cmp(&self, other: &Value, system_data: &SystemData) -> std::cmp::Ordering {
        let value_a = self.value_in("USD", system_data);
        let value_b = other.value_in("USD", system_data);
        value_a
            .quantity
            .partial_cmp(&value_b.quantity)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl From<(usize, &str)> for Value {
    fn from(input: (usize, &str)) -> Self {
        Self {
            quantity: input.0 as f64,
            currency_symbol: input.1.to_string(),
        }
    }
}

impl From<(f64, &str)> for Value {
    fn from(input: (f64, &str)) -> Self {
        Self {
            quantity: input.0,
            currency_symbol: input.1.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct TransactionValue {
    pub wallet: String,
    pub value: Value,
}

impl From<(&str, Value)> for TransactionValue {
    fn from(input: (&str, Value)) -> Self {
        Self {
            wallet: input.0.to_string(),
            value: input.1,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub from: TransactionValue,
    pub to: TransactionValue,
    pub exchange: Option<String>,
    pub fee: Option<Value>,
    pub ts: DateTime<Utc>,
}

impl From<(&str, &str, Value, Value, Option<&str>, Option<Value>, &str)> for Transaction {
    fn from(input: (&str, &str, Value, Value, Option<&str>, Option<Value>, &str)) -> Self {
        Self {
            from: (input.0, input.2).into(),
            to: (input.1, input.3).into(),
            exchange: input.4.map(Into::into),
            fee: input.5,
            ts: input.6.parse().unwrap(),
        }
    }
}

pub struct Exchange {
    pub id: String,
    pub name: String,
    pub short_name: Option<String>,
    pub url: String,
}

impl Exchange {
    pub fn get_name(&self, short: bool) -> String {
        if short {
            self.short_name.clone().unwrap_or(self.name.clone())
        } else {
            self.name.clone()
        }
    }
}

impl From<(&str, &str, Option<&str>, &str)> for Exchange {
    fn from(input: (&str, &str, Option<&str>, &str)) -> Self {
        Self {
            id: input.0.to_string(),
            name: input.1.to_string(),
            short_name: input.2.map(ToString::to_string),
            url: input.3.to_string(),
        }
    }
}

pub struct Wallet {
    pub id: String,
    pub name: String,
    pub short_name: Option<String>,
    pub url: String,
}

impl Wallet {
    pub fn get_name(&self, short: bool) -> String {
        if short {
            self.short_name.clone().unwrap_or(self.name.clone())
        } else {
            self.name.clone()
        }
    }
}

impl From<(&str, &str, Option<&str>, &str)> for Wallet {
    fn from(input: (&str, &str, Option<&str>, &str)) -> Self {
        Self {
            id: input.0.to_string(),
            name: input.1.to_string(),
            short_name: input.2.map(ToString::to_string),
            url: input.3.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PortfolioEntryState {
    pub value: Value,
    pub roi: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PortfolioState {
    pub entries: HashMap<String, PortfolioEntryState>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Portfolio {
    pub name: String,
    pub currencies: Option<Vec<String>>,
    pub state: RefCell<Option<PortfolioState>>,
}

impl Portfolio {
    pub fn calculate_state(&self, transactions: &[Transaction], system_data: &SystemData) {
        if self.state.borrow().is_some() {
            return;
        }

        let mut currencies: HashMap<String, PortfolioEntryState> = HashMap::new();

        currencies.insert(
            "BTC".to_string(),
            PortfolioEntryState {
                value: (0.0, "BTC").into(),
                roi: 0.0,
            },
        );

        for t in transactions {
            if t.to.value.currency_symbol == "BTC" {
                let current_price = system_data
                    .get_price(&t.to.value.currency_symbol, &t.from.value.currency_symbol)
                    .unwrap();
                let value = t.to.value.quantity * current_price;
                let paid = t.from.value.quantity;
                let roi = value / paid;
                let mut entry = currencies.get_mut("BTC").unwrap();
                entry.roi += roi - 1.0;
                entry.value.quantity += t.to.value.quantity;
            }
        }
        let mut state = self.state.borrow_mut();

        *state = Some(PortfolioState {
            entries: currencies,
        });
    }

    pub fn total_value(&self, symbol: &str, system_data: &SystemData) -> Value {
        let mut result = 0.0;

        let state = self.state.borrow();

        for (_, entry) in &state.as_ref().unwrap().entries {
            result += entry.value.value_in(symbol, system_data).quantity;
        }

        (result, symbol).into()
    }
}

impl From<(&str, Option<Vec<&str>>)> for Portfolio {
    fn from(input: (&str, Option<Vec<&str>>)) -> Self {
        Self {
            name: input.0.to_string(),
            currencies: input
                .1
                .map(|curr| curr.into_iter().map(Into::into).collect()),
            state: RefCell::new(None),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Price {
    pub pair: (String, String),
    pub value: f64,
}

impl From<(&str, &str, f64)> for Price {
    fn from(input: (&str, &str, f64)) -> Self {
        Self {
            pair: (input.0.to_string(), input.1.to_string()),
            value: input.2,
        }
    }
}
