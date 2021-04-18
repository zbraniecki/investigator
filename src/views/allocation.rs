use super::ViewType;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    fs,
};
use crate::{
    session::Session,
    system::System,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};
use termion::event::Key;
use float_pretty_print::PrettyPrintFloat;

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
        let name = "allocation";
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

    fn render(&self,
              frame: &mut Frame<B>,
              area: Rect,
              session: &Session,
              system: &System) {
        let header_cells = self
            .columns
            .iter()
            .map(|col| Cell::from(col.0.clone()).style(Style::default().fg(Color::White)));
        let header = Row::new(header_cells)
            .style(Style::default().add_modifier(Modifier::BOLD | Modifier::DIM))
            .height(1)
            .bottom_margin(1);

        let allocation = &session.state.borrow().allocations[0];

        let mut values: Vec<_> = system.data.tokens.iter().map(|token| {
            (
                token.symbol.clone(),
                allocation.values
                .iter()
                .find(|(name, _)| *name == &token.symbol)
                .map(|(_, value)| *value)
            )
        }).collect();

        use std::cmp::Ordering;
        values.sort_by(|v1, v2| {
            match (v1.1, v2.1) {
                (Some(v1), Some(v2)) => v1.partial_cmp(&v2).unwrap_or(Ordering::Equal).reverse(),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            }
        });

        let rows:Vec<_> = values
            .iter()
            .map(|(symbol, value)| {
                let value = if let Some(value) = value {
                    format!("{}%", PrettyPrintFloat(*value * 100.0))
                } else {
                    "_".to_string()
                };
                Row::new(vec![
                    Cell::from(symbol.to_string()),
                    Cell::from(value),
                ])
            }).collect();
        self.state.borrow_mut().len = rows.len();

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

        frame.render_stateful_widget(table, area, &mut self.state.borrow_mut().table_state);
    }

    fn handle_event(&self, key: &Key, session: &Session, system: &System) {
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
            }
            Key::Up => {
                let mut state = self.state.borrow_mut();

                let i = match state.table_state.selected() {
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
            }
            Key::Char(ch) => {
                if ch.is_ascii_digit() {
                    let state = self.state.borrow();
                    if let Some(idx) = state.table_state.selected() {
                        let token = system.data.tokens.get(idx).unwrap();
                        let allocation = &mut session.state.borrow_mut().allocations[0];
                        allocation.values.entry(token.symbol.to_string())
                            .and_modify(|value| {
                                *value *= 10.0;
                                *value += ch.to_digit(10).unwrap() as f64 / 100.0;
                            })
                            .or_insert(ch.to_digit(10).unwrap() as f64 / 100.0);
                    }
                }
            }
            Key::Backspace => {
                let state = self.state.borrow();
                if let Some(idx) = state.table_state.selected() {
                    let token = system.data.tokens.get(idx).unwrap();
                    let allocation = &mut session.state.borrow_mut().allocations[0];
                    allocation.values.remove(&token.symbol);
                }
            }
            _ => {}
        }
    }
}
