use super::ViewType;
use crate::{app::App, ui::event::HandleEventResult};
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
        let name = "portfolio";
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

        let portfolio = &app.session.state.borrow().portfolios[0];
        let reference_currency = "USD";
        // portfolio.calculate_state(transactions, app.get_system_data());
        let total_value = portfolio.total_value(reference_currency, &app.system.data);

        let pstate = portfolio.state.borrow();
        let currencies = pstate.as_ref().unwrap();

        let mut result: Vec<_> = currencies.entries.iter().map(|(_, v)| v).collect();
        result.sort_by(|a, b| a.value.cmp(&b.value, &app.system.data));
        result.reverse();

        let height = 1;
        let rows: Vec<_> = result
            .into_iter()
            .map(|entry| {
                let value_in_ref = entry.value.value_in(reference_currency, &app.system.data);
                let percent_value = value_in_ref.quantity / total_value.quantity;
                let roi = entry.roi * percent_value * 100.0;

                Row::new(vec![
                    Cell::from(entry.value.currency_symbol.clone()),
                    Cell::from(format!("{:.2}%", percent_value * 100.0)),
                    Cell::from(format!("{}", PrettyPrintFloat(entry.value.quantity))),
                    Cell::from(value_in_ref.format_with_precision(true, 2)),
                    Cell::from(format!("{:.2}%", roi)),
                ])
                .height(height as u16)
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
