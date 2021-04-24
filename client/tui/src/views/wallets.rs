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
        let name = "wallets";
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

        let state = app.session.state.borrow();
        let wallets = &state.wallets;

        let rows: Vec<_> = wallets
            .iter()
            .map(|item| {
                let exchange = app.system.data.get_wallet(item).unwrap();
                let height = 1;
                let cells = get_row_from_wallet(exchange);
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

fn get_row_from_wallet<'s, 'l>(t: &'s model::Wallet) -> Vec<Cell<'l>> {
    vec![
        Cell::from(t.name.to_string()),
        Cell::from(t.url.to_string()),
    ]
}
