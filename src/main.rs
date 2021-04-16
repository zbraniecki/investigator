mod app;
mod event;
mod model;
mod state;
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
    widgets::{Block, Borders, Cell, Row, Table, Tabs},
    Terminal,
};

fn get_menu_item(name: &str) -> Spans {
    let (first, rest) = name.split_at(1);
    Spans::from(vec![
        Span::styled(first, Style::default().add_modifier(Modifier::UNDERLINED)),
        Span::styled(rest, Style::default().fg(Color::White)),
    ])
}

fn get_table<'s, 'l>(
    view: &'s views::View,
    app: &'s App,
) -> (Row<'l>, Vec<Row<'l>>, Vec<Constraint>) {
    let header_cells = view
        .columns
        .iter()
        .map(|h| Cell::from(h.to_string()).style(Style::default().fg(Color::White)));
    let header = Row::new(header_cells)
        .style(Style::default().add_modifier(Modifier::BOLD | Modifier::DIM))
        .height(1)
        .bottom_margin(1);
    let rows = view.get_rows(app);

    let widths: Vec<Constraint> = view
        .widths
        .iter()
        .map(|width| Constraint::Percentage(*width as u16))
        .collect();
    (header, rows, widths)
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
                .map(|view| get_menu_item(&view.name))
                .collect();
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

            let (header, rows, widths) = get_table(active_view, &app);

            let selected_style = Style::default().fg(Color::Yellow);
            let t = Table::new(rows)
                .header(header)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().add_modifier(Modifier::DIM)),
                )
                .highlight_style(selected_style)
                .widths(&widths);

            let mut view_state = app.ui.get_active_view_state();
            f.render_stateful_widget(t, chunks[1], &mut view_state);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Down => {
                    app.next();
                }
                Key::Up => {
                    app.previous();
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
                Key::Char(c) => {
                    let views = &app.ui.views;
                    for (idx, view) in views.iter().enumerate() {
                        if view.menu_key == c {
                            app.ui.state.active_menu_idx = idx;
                        }
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
