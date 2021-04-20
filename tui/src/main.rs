mod app;
mod model;
mod session;
mod system;
mod ui;
mod views;

use app::App;
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
use ui::event::{Event, Events, HandleEventResult};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let app = App::new();
    app.ui.views[0].before_display(&app);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(3), Constraint::Percentage(95)].as_ref())
                .split(f.size());

            app.ui.menu.render(f, chunks[0], &app);

            let active_view = app.ui.get_active_view();

            active_view.render(f, chunks[1], &app);
        })?;

        match events.next()? {
            Event::Input(input) => match app.ui.handle_event(&input, &app) {
                HandleEventResult::Handled => {}
                HandleEventResult::Bubbled => {}
                HandleEventResult::Quit => {
                    break;
                }
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//   We'll need an event loop here
//     Ok(())
// }
