use crate::model;
use crate::App;
use chrono_tz::US::Pacific;
use std::collections::HashMap;
use tui::widgets::{Cell, Row};

pub struct View {
    pub name: String,
    pub columns: Vec<String>,
    pub widths: Vec<usize>,
    pub menu_key: char,
}

fn get_row_from_exchange<'s, 'l>(t: &'s model::Exchange) -> Vec<Cell<'l>> {
    vec![
        Cell::from(t.name.to_string()),
        Cell::from(t.url.to_string()),
    ]
}

fn get_row_from_wallet<'s, 'l>(t: &'s model::Wallet) -> Vec<Cell<'l>> {
    vec![
        Cell::from(t.name.to_string()),
        Cell::from(t.url.to_string()),
    ]
}

fn get_row_from_transaction<'s, 'l>(t: &'s model::Transaction, app: &App) -> Vec<Cell<'l>> {
    let date =
        t.ts.with_timezone(&Pacific)
            .format("%Y-%m-%d %H:%M")
            .to_string();
    let wallet_to = app.get_system_data().get_wallet(&t.to.wallet).unwrap();
    let wallet_from = app.get_system_data().get_wallet(&t.from.wallet).unwrap();
    let exchange = t
        .exchange
        .as_ref()
        .map(|eid| app.get_system_data().get_exchange(&eid).unwrap());
    let price = model::Value::from((
        t.from.value.quantity / t.to.value.quantity,
        t.from.value.currency_symbol.as_str(),
    ))
    .format_with_precision(true, 2);
    vec![
        Cell::from(t.to.value.format(false)),
        Cell::from(t.from.value.format(false)),
        Cell::from(price),
        Cell::from(wallet_to.get_name(true)),
        Cell::from(wallet_from.get_name(true)),
        Cell::from(
            exchange
                .as_ref()
                .map(|e| e.get_name(true))
                .unwrap_or("".to_string()),
        ),
        Cell::from(date),
    ]
}

struct PortfolioEntry {
    currency_symbol: String,
    quantity: f64,
    roi: f64,
}

fn calculate_portfolio<'s, 'l>(
    p: &'s model::Portfolio,
    transactions: &'s Vec<model::Transaction>,
) -> Vec<Row<'l>> {
    let mut currencies: HashMap<String, PortfolioEntry> = HashMap::new();

    currencies.insert(
        "BTC".to_string(),
        PortfolioEntry {
            currency_symbol: "BTC".to_string(),
            quantity: 0.0,
            roi: 0.0,
        },
    );

    for t in transactions {
        if t.to.value.currency_symbol == "BTC" {
            let mut entry = currencies.get_mut("BTC").unwrap();
            entry.quantity += t.to.value.quantity;
        }
    }

    let mut result: Vec<_> = currencies.into_iter().map(|(_, v)| v).collect();
    result.sort_by(|a, b| {
        a.roi
            .partial_cmp(&b.roi)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    result.reverse();

    let height = 1;
    result
        .into_iter()
        .map(|entry| {
            Row::new(vec![
                Cell::from(entry.currency_symbol),
                Cell::from(entry.quantity.to_string()),
                Cell::from("0"),
                Cell::from(format!("{}%", entry.roi)),
            ])
            .height(height as u16)
        })
        .collect()
}

impl View {
    pub fn get_rows<'s, 'l>(&self, app: &'s App) -> Vec<Row<'l>> {
        match self.name.as_str() {
            "Exchanges" => app
                .state
                .exchanges
                .iter()
                .map(|item| {
                    let exchange = app.get_system_data().get_exchange(item).unwrap();
                    let height = 1;
                    let cells = get_row_from_exchange(exchange);
                    Row::new(cells).height(height as u16)
                })
                .collect(),
            "Wallets" => app
                .state
                .wallets
                .iter()
                .map(|item| {
                    let wallet = app.get_system_data().get_wallet(item).unwrap();
                    let height = 1;
                    let cells = get_row_from_wallet(wallet);
                    Row::new(cells).height(height as u16)
                })
                .collect(),
            "Portfolio" => {
                let portfolio = &app.state.portfolios[0];
                calculate_portfolio(portfolio, &app.state.transactions)
            }
            "Transactions" => app
                .state
                .transactions
                .iter()
                .map(|item| {
                    let height = 1;
                    let cells = get_row_from_transaction(item, app);
                    Row::new(cells).height(height as u16)
                })
                .collect(),
            _ => {
                vec![]
            }
        }
    }
}

pub fn get_all() -> Vec<View> {
    vec![
        View {
            name: "Exchanges".to_string(),
            columns: vec![
                "Name".to_string(),
                "URL".to_string(),
                "Currencies".to_string(),
            ],
            widths: vec![20, 30, 50],
            menu_key: 'e',
        },
        View {
            name: "Wallets".to_string(),
            columns: vec![
                "Name".to_string(),
                "URL".to_string(),
                "Currencies".to_string(),
            ],
            widths: vec![20, 30, 50],
            menu_key: 'w',
        },
        View {
            name: "Portfolio".to_string(),
            columns: vec![
                "Symbol".to_string(),
                "Quantity".to_string(),
                "Value in USD".to_string(),
                "ROI".to_string(),
            ],
            widths: vec![10, 30, 30, 30],
            menu_key: 'p',
        },
        View {
            name: "Transactions".to_string(),
            columns: vec![
                "Bought".to_string(),
                "Paid".to_string(),
                "Price".to_string(),
                "Wallet".to_string(),
                "From".to_string(),
                "Exchange".to_string(),
                "Date".to_string(),
            ],
            widths: vec![19, 19, 14, 8, 8, 10, 20],
            menu_key: 't',
        },
    ]
}
