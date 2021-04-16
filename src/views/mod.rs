use crate::model;
use crate::App;
use chrono::prelude::*;
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

fn get_row_from_transaction<'s, 'l>(t: &'s model::Transaction) -> Vec<Cell<'l>> {
    let price = t.value_to.quantity / t.value_from.quantity;
    let dt = Utc.ymd(2021, 3, 13).and_hms(9, 10, 11);
    let date = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    vec![
        Cell::from(t.value_to.to_string()),
        Cell::from(t.value_from.to_string()),
        Cell::from(price.to_string()),
        Cell::from(t.wallet_to_id.to_string()),
        Cell::from(t.wallet_from_id.to_string()),
        Cell::from(
            t.exchange_id
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_string(),
        ),
        Cell::from(date),
    ]
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
                let _portfolio = &app.state.portfolios[0];
                let height = 1;
                vec![Row::new(vec![
                    Cell::from("BTC"),
                    Cell::from("1"),
                    Cell::from("$100"),
                    Cell::from("3.4%"),
                ])
                .height(height as u16)]
            }
            "Transactions" => app
                .state
                .transactions
                .iter()
                .map(|item| {
                    let height = 1;
                    let cells = get_row_from_transaction(item);
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
            widths: vec![10, 10, 10, 15, 10, 10, 25],
            menu_key: 't',
        },
    ]
}
