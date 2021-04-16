use crate::model;
use crate::App;
use chrono_tz::US::Pacific;
use float_pretty_print::PrettyPrintFloat;
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

fn calculate_portfolio<'s, 'l>(
    p: &'s model::Portfolio,
    transactions: &'s Vec<model::Transaction>,
    app: &App,
) -> Vec<Row<'l>> {
    let reference_currency = "USD";

    p.calculate_state(transactions, app.get_system_data());

    let total_value = p.total_value(reference_currency, app.get_system_data());

    let state = p.state.borrow();
    let currencies = state.as_ref().unwrap();

    let mut result: Vec<_> = currencies.entries.iter().map(|(_, v)| v).collect();
    result.sort_by(|a, b| a.value.cmp(&b.value, app.get_system_data()));
    result.reverse();

    let height = 1;
    result
        .into_iter()
        .map(|entry| {
            let value_in_ref = entry
                .value
                .value_in(reference_currency, app.get_system_data());
            let percent_value = value_in_ref.quantity / total_value.quantity * 100.0;
            let roi = entry.roi * 100.0;

            Row::new(vec![
                Cell::from(entry.value.currency_symbol.clone()),
                Cell::from(format!("{:.2}%", percent_value)),
                Cell::from(format!("{}", PrettyPrintFloat(entry.value.quantity))),
                Cell::from(value_in_ref.format_with_precision(true, 2)),
                Cell::from(format!("{}%", PrettyPrintFloat(roi))),
            ])
            .height(height as u16)
        })
        .collect()
}

fn get_transactions(app: &App) -> Vec<model::Transaction> {
    let mut result = vec![];

    let mut iter = app.state.transactions.iter().peekable();
    while let Some(t1) = iter.next() {
        if let Some(t2) = iter.peek() {
            if t1.from == t2.to && t2.exchange.is_none() {
                let mut fee = t1.fee.clone();
                if let Some(f2) = &t2.fee {
                    if let Some(ref mut f1) = &mut fee {
                        if f1.currency_symbol == f2.currency_symbol {
                            f1.quantity += f2.quantity;
                        }
                    }
                }
                result.push(model::Transaction {
                    from: t2.from.clone(),
                    to: t1.to.clone(),
                    exchange: t1.exchange.clone(),
                    fee,
                    ts: t1.ts.clone(),
                });
                iter.next();
                continue;
            }
        }
        result.push(t1.clone());
    }

    result
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
                calculate_portfolio(portfolio, &app.state.transactions, app)
            }
            "Transactions" => {
                let transactions = get_transactions(app);
                transactions
                    .iter()
                    .map(|item| {
                        let height = 1;
                        let cells = get_row_from_transaction(item, app);
                        Row::new(cells).height(height as u16)
                    })
                    .collect()
            }
            _ => {
                vec![]
            }
        }
    }
}

pub fn get_all() -> Vec<View> {
    vec![
        View {
            name: "Portfolio".to_string(),
            columns: vec![
                "Symbol".to_string(),
                "%".to_string(),
                "Quantity".to_string(),
                "Value in USD".to_string(),
                "ROI".to_string(),
            ],
            widths: vec![10, 10, 30, 30, 20],
            menu_key: 'p',
        },
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
