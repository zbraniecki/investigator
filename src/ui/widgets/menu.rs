use crate::{app::App, ui::event::HandleEventResult};
use std::cell::RefCell;
use termion::event::Key;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Row, Table, Tabs},
};

pub struct MenuState {}

impl MenuState {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Menu {
    pub state: RefCell<MenuState>,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(MenuState::new()),
        }
    }

    pub fn render<B>(&self, frame: &mut Frame<B>, area: Rect, app: &App<B>)
    where
        B: Backend,
    {
        let mut menu: Vec<Spans> = app
            .ui
            .views
            .iter()
            .map(|view| get_menu_item(view.get_name()))
            .collect();
        menu.push(get_menu_item("Save"));
        menu.push(get_menu_item("Quit"));

        let tabs = Tabs::new(menu)
            .select(app.ui.get_active_view_idx())
            .block(Block::default().borders(Borders::ALL))
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::DIM),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .remove_modifier(Modifier::DIM),
            )
            .divider(Span::raw("|"));

        frame.render_widget(tabs, area);
    }

    pub fn handle_event<B>(&self, key: &Key, app: &App<B>) -> HandleEventResult
    where
        B: Backend,
    {
        match key {
            Key::Char('q') => HandleEventResult::Quit,
            Key::Char('s') => {
                app.session.state.borrow().save();
                HandleEventResult::Handled
            }
            Key::Char('\t') => {
                let views = app.ui.get_view_names();
                let mut active_view_idx = app.ui.get_active_view_idx();

                if active_view_idx < views.len() - 1 {
                    active_view_idx += 1;
                } else {
                    active_view_idx = 0;
                }
                app.ui.set_view(&views[active_view_idx], app);
                HandleEventResult::Handled
            }
            Key::BackTab => {
                let views = app.ui.get_view_names();
                let mut active_view_idx = app.ui.get_active_view_idx();

                if active_view_idx == 0 {
                    active_view_idx = views.len() - 1;
                } else {
                    active_view_idx -= 1;
                }
                app.ui.set_view(&views[active_view_idx], app);
                HandleEventResult::Handled
            }
            _ => HandleEventResult::Bubbled,
        }
    }
}

fn get_menu_item(name: &str) -> Spans {
    let (first, rest) = name.split_at(1);
    Spans::from(vec![
        Span::styled(first, Style::default().add_modifier(Modifier::UNDERLINED)),
        Span::styled(rest, Style::default().fg(Color::White)),
    ])
}
