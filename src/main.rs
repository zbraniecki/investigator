mod app;
mod event;
mod model;
mod session;
mod system;
mod ui;
mod views;

use app::App;
use event::{Event, Events};
use std::{error::Error, io};
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Terminal,
};

fn get_menu_item(name: &str) -> Spans {
    let (first, rest) = name.split_at(1);
    Spans::from(vec![
        Span::styled(first, Style::default().add_modifier(Modifier::UNDERLINED)),
        Span::styled(rest, Style::default().fg(Color::White)),
    ])
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(3), Constraint::Percentage(95)].as_ref())
                .split(f.size());

            let mut menu: Vec<Spans> = app
                .ui
                .views
                .iter()
                .map(|view| get_menu_item(view.get_name()))
                .collect();
            menu.push(get_menu_item("Save"));
            menu.push(get_menu_item("Quit"));

            let tabs = Tabs::new(menu)
                .select(app.ui.state.active_menu_idx)
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

            f.render_widget(tabs, chunks[0]);

            let active_view = app.ui.get_active_view();

            active_view.render(f, chunks[1], &app.session, &app.system);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('s') => {
                    app.session.state.borrow().save();
                }
                Key::Char('\t') => {
                    let views = &app.ui.views;
                    if app.ui.state.active_menu_idx < views.len() - 1 {
                        app.ui.state.active_menu_idx += 1;
                    } else {
                        app.ui.state.active_menu_idx = 0;
                    }
                }
                Key::BackTab => {
                    let views = &app.ui.views;
                    if app.ui.state.active_menu_idx == 0 {
                        app.ui.state.active_menu_idx = views.len() - 1;
                    } else {
                        app.ui.state.active_menu_idx -= 1;
                    }
                }
                key @ _ => {
                    let active_view = app.ui.get_active_view();
                    active_view.handle_event(&key, &app.session, &app.system);
                }
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
