use super::ViewType;
use crate::{app::App, model, ui::event::HandleEventResult};
use chrono_tz::US::Pacific;
use float_pretty_print::PrettyPrintFloat;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, fs};
use termion::event::Key;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};

#[derive(Serialize, Deserialize)]
pub struct ViewState {
    #[serde(skip)]
    pub table_state: TableState,
    pub selected_column: usize,
    pub len: usize,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            table_state: TableState::default(),
            selected_column: 1,
            len: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct View {
    pub name: String,
    pub columns: Vec<(String, usize)>,
    pub menu_key: char,
    #[serde(default)]
    pub state: RefCell<ViewState>,
}

impl View {
    pub fn new() -> Self {
        let name = "transactions";
        let source = fs::read_to_string(&format!("res/ui/{}.toml", name))
            .expect("Something went wrong reading the file");
        toml::from_str(&source).unwrap()
    }
}

impl<B> ViewType<B> for View
where
    B: Backend,
{
    fn get_name(&self) -> &str {
        &self.name
    }

    fn render(&self, frame: &mut Frame<B>, area: Rect, app: &App<B>) {
        let header_cells = self
            .columns
            .iter()
            .map(|col| Cell::from(col.0.clone()).style(Style::default().fg(Color::White)));
        let header = Row::new(header_cells)
            .style(Style::default().add_modifier(Modifier::BOLD | Modifier::DIM))
            .height(1)
            .bottom_margin(1);

        let transactions = get_transactions(app);
        let rows: Vec<_> = transactions
            .iter()
            .map(|item| {
                let height = 1;
                let cells = get_row_from_transaction(item, app);
                Row::new(cells).height(height as u16)
            })
            .collect();

        let mut state = self.state.borrow_mut();
        state.len = rows.len();

        let widths: Vec<Constraint> = self
            .columns
            .iter()
            .map(|col| Constraint::Percentage(col.1 as u16))
            .collect();

        let selected_style = Style::default().fg(Color::Yellow);
        let table = Table::new(rows)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().add_modifier(Modifier::DIM)),
            )
            .highlight_style(selected_style)
            .widths(&widths);

        frame.render_stateful_widget(table, area, &mut state.table_state);
    }

    fn handle_event(&self, key: &Key, app: &App<B>) -> HandleEventResult {
        match key {
            Key::Down => {
                let mut state = self.state.borrow_mut();

                let i = match state.table_state.selected() {
                    Some(i) => {
                        if i >= state.len - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                state.table_state.select(Some(i));
                HandleEventResult::Handled
            }
            Key::Up => {
                let mut state = self.state.borrow_mut();

                let selected_idx = state.table_state.selected();

                let i = match selected_idx {
                    Some(i) => {
                        if i == 0 {
                            state.len - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                state.table_state.select(Some(i));
                HandleEventResult::Handled
            }
            _ => HandleEventResult::Bubbled,
        }
    }
}

fn get_transactions<B>(app: &App<B>) -> Vec<model::Transaction>
where
    B: Backend,
{
    let mut result = vec![];

    let state = app.session.state.borrow();
    let mut iter = state.transactions.iter().peekable();
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

fn get_row_from_transaction<'s, 'l, B>(t: &'s model::Transaction, app: &App<B>) -> Vec<Cell<'l>>
where
    B: Backend,
{
    let date =
        t.ts.with_timezone(&Pacific)
            .format("%Y-%m-%d %H:%M")
            .to_string();
    let system_data = &app.system.data;
    let wallet_to = system_data.get_wallet(&t.to.wallet).unwrap();
    let wallet_from = system_data.get_wallet(&t.from.wallet).unwrap();
    let exchange = t
        .exchange
        .as_ref()
        .map(|eid| system_data.get_exchange(&eid).unwrap());
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
