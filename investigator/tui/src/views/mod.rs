mod allocation;
mod exchanges;
mod portfolio;
mod transactions;
mod wallets;
mod market;

// use crate::model;
// use crate::App;
// use chrono_tz::US::Pacific;
// use float_pretty_print::PrettyPrintFloat;
// use tui::widgets::{Cell, Row};
use crate::{ui::event::HandleEventResult, App};
use termion::event::Key;
use tui::{
    backend::Backend,
    layout::Rect,
    terminal::Frame,
};

pub trait ViewType<B>
where
    B: Backend,
{
    fn get_name(&self) -> &str;
    fn render(&self, frame: &mut Frame<B>, area: Rect, app: &App<B>);
    fn before_display(&self, _app: &App<B>) {}
    fn before_hide(&self, _app: &App<B>) {}
    fn handle_event(&self, event: &Key, app: &App<B>) -> HandleEventResult;
}

pub fn get_all<B>() -> Vec<Box<dyn ViewType<B>>>
where
    B: Backend,
{
    vec![
        Box::new(market::View::new()),
        Box::new(portfolio::View::new()),
        Box::new(allocation::View::new()),
        Box::new(transactions::View::new()),
        Box::new(exchanges::View::new()),
        Box::new(wallets::View::new()),
    ]
}
