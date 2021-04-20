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
        let name = "allocation";
        let source = fs::read_to_string(&format!("res/ui/{}.toml", name))
            .expect("Something went wrong reading the file");
        toml::from_str(&source).unwrap()
    }

    pub fn update_input<B>(&self, selected_idx: usize, app: &App<B>)
    where
        B: Backend,
    {
        app.ui.input.activate();

        let token = &app.system.data.tokens[selected_idx];
        let mut session_state = app.session.state.borrow_mut();
        let allocation = &mut session_state.allocations[0];

        app.ui.input.set_value(
            allocation
                .values
                .get(&token.symbol)
                .copied()
                .map(|v| v * 100.0),
        );
    }

    pub fn update_value<B>(&self, selected_idx: usize, app: &App<B>)
    where
        B: Backend,
    {
        if app.ui.input.is_active() && app.ui.input.interacted() {
            if let Some(v) = app.ui.input.value() {
                let token = &app.system.data.tokens[selected_idx];
                let mut session_state = app.session.state.borrow_mut();
                let allocation = &mut session_state.allocations[0];

                if let Some(value) = allocation.values.get_mut(&token.symbol) {
                    *value = v / 100.0;
                } else {
                    allocation.values.insert(token.symbol.clone(), v / 100.0);
                }
            } else {
                let token = &app.system.data.tokens[selected_idx];
                let mut session_state = app.session.state.borrow_mut();
                let allocation = &mut session_state.allocations[0];
                allocation.values.remove(&token.symbol);
            }
        }
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

        let allocation = &app.session.state.borrow().allocations[0];

        let mut values: Vec<_> = app
            .system
            .data
            .tokens
            .iter()
            .map(|token| {
                (
                    token.symbol.clone(),
                    allocation
                        .values
                        .iter()
                        .find(|(name, _)| *name == &token.symbol)
                        .map(|(_, value)| *value),
                )
            })
            .collect();

        // use std::cmp::Ordering;
        // values.sort_by(|v1, v2| {
        //     match (v1.1, v2.1) {
        //         (Some(v1), Some(v2)) => v1.partial_cmp(&v2).unwrap_or(Ordering::Equal).reverse(),
        //         (Some(_), None) => Ordering::Less,
        //         (None, Some(_)) => Ordering::Greater,
        //         (None, None) => Ordering::Equal,
        //     }
        // });
        let mut state = self.state.borrow_mut();

        let active_row = state.table_state.selected();

        let rows: Vec<_> = values
            .iter()
            .enumerate()
            .map(|(idx, (symbol, value))| {
                let value = if Some(idx) == active_row {
                    app.ui
                        .input
                        .value()
                        .map(|v| format!("{}%", PrettyPrintFloat(v)))
                        .unwrap_or("_".to_string())
                } else {
                    if let Some(value) = value {
                        format!("{}%", PrettyPrintFloat(*value * 100.0))
                    } else {
                        "_".to_string()
                    }
                };
                Row::new(vec![Cell::from(symbol.to_string()), Cell::from(value)])
            })
            .collect();
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
                        self.update_value(i, app);
                        if i >= state.len - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                state.table_state.select(Some(i));
                self.update_input(i, app);
                HandleEventResult::Handled
            }
            Key::Up => {
                let mut state = self.state.borrow_mut();

                let selected_idx = state.table_state.selected();

                let i = match selected_idx {
                    Some(i) => {
                        self.update_value(i, app);
                        if i == 0 {
                            state.len - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                state.table_state.select(Some(i));
                self.update_input(i, app);
                HandleEventResult::Handled
            }
            _ => HandleEventResult::Bubbled,
        }
    }
}
