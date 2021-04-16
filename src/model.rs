use std::fmt::{Display, Formatter};

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

pub struct Value {
    pub currency_symbol: String,
    pub quantity: usize,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} {}", self.quantity, self.currency_symbol)
    }
}

impl From<(usize, &str)> for Value {
    fn from(input: (usize, &str)) -> Self {
        Self {
            currency_symbol: input.1.to_string(),
            quantity: input.0,
        }
    }
}

pub struct Transaction {
    pub wallet_from_id: String,
    pub wallet_to_id: String,
    pub value_from: Value,
    pub value_to: Value,
    pub exchange_id: Option<String>,
    pub fee: Option<Value>,
    pub date: usize,
}

impl From<(&str, &str, Value, Value, Option<&str>, Option<Value>, usize)> for Transaction {
    fn from(input: (&str, &str, Value, Value, Option<&str>, Option<Value>, usize)) -> Self {
        Self {
            wallet_from_id: input.0.to_string(),
            wallet_to_id: input.1.to_string(),
            value_from: input.2,
            value_to: input.3,
            exchange_id: input.4.map(Into::into),
            fee: input.5,
            date: input.6,
        }
    }
}

pub struct Exchange {
    pub id: String,
    pub name: String,
    pub url: String,
}

impl From<(&str, &str, &str)> for Exchange {
    fn from(input: (&str, &str, &str)) -> Self {
        Self {
            id: input.0.to_string(),
            name: input.1.to_string(),
            url: input.2.to_string(),
        }
    }
}

pub struct Wallet {
    pub id: String,
    pub name: String,
    pub url: String,
}

impl From<(&str, &str, &str)> for Wallet {
    fn from(input: (&str, &str, &str)) -> Self {
        Self {
            id: input.0.to_string(),
            name: input.1.to_string(),
            url: input.1.to_string(),
        }
    }
}

pub struct Portfolio {
    pub name: String,
    pub currencies: Option<Vec<String>>,
}

impl From<(&str, Option<Vec<&str>>)> for Portfolio {
    fn from(input: (&str, Option<Vec<&str>>)) -> Self {
        Self {
            name: input.0.to_string(),
            currencies: input
                .1
                .map(|curr| curr.into_iter().map(Into::into).collect()),
        }
    }
}
