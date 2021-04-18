mod allocation;
mod exchanges;
mod portfolio;
mod transactions;
mod wallets;

// use crate::model;
// use crate::App;
// use chrono_tz::US::Pacific;
// use float_pretty_print::PrettyPrintFloat;
// use tui::widgets::{Cell, Row};
use crate::{session::Session, system::System, ui::event::HandleEventResult, App};
use serde::{Deserialize, Serialize};
use termion::event::Key;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    terminal::Frame,
    widgets::{Cell, Row, Table},
};

pub trait ViewType<B>
where
    B: Backend,
{
    fn get_name(&self) -> &str;
    fn render(&self, frame: &mut Frame<B>, area: Rect, app: &App<B>);
    fn before_display(&self, app: &App<B>) {}
    fn before_hide(&self, app: &App<B>) {}
    fn handle_event(&self, event: &Key, app: &App<B>) -> HandleEventResult;
}

// fn get_row_from_wallet<'s, 'l>(t: &'s model::Wallet) -> Vec<Cell<'l>> {
//     vec![
//         Cell::from(t.name.to_string()),
//         Cell::from(t.url.to_string()),
//     ]
// }

// impl View {
//     pub fn get_rows<'s, 'l>(&self, app: &'s App) -> Vec<Row<'l>> {
//         match self.name.as_str() {
//             "Exchanges" => app
//                 .state
//                 .exchanges
//                 .iter()
//                 .map(|item| {
//                     let exchange = app.get_system_data().get_exchange(item).unwrap();
//                     let height = 1;
//                     let cells = get_row_from_exchange(exchange);
//                     Row::new(cells).height(height as u16)
//                 })
//                 .collect(),
//             "Wallets" => app
//                 .state
//                 .wallets
//                 .iter()
//                 .map(|item| {
//                     let wallet = app.get_system_data().get_wallet(item).unwrap();
//                     let height = 1;
//                     let cells = get_row_from_wallet(wallet);
//                     Row::new(cells).height(height as u16)
//                 })
//                 .collect(),
//             "Portfolio" => {
//                 let portfolio = &app.state.portfolios[0];
//                 calculate_portfolio(portfolio, &app.state.transactions, app)
//             }
//             "Transactions" => {
//                 let transactions = get_transactions(app);
//                 transactions
//                     .iter()
//                     .map(|item| {
//                         let height = 1;
//                         let cells = get_row_from_transaction(item, app);
//                         Row::new(cells).height(height as u16)
//                     })
//                     .collect()
//             }
//             _ => {
//                 vec![]
//             }
//         }
//     }
// }

pub fn get_all<B>() -> Vec<Box<dyn ViewType<B>>>
where
    B: Backend,
{
    vec![
        Box::new(allocation::View::new()),
        Box::new(portfolio::View::new()),
        Box::new(transactions::View::new()),
        Box::new(exchanges::View::new()),
        Box::new(wallets::View::new()),
    ]
}
