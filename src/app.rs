use crate::system::{System, SystemData};
use crate::session::Session;
use crate::ui::UI;
use tui::backend::Backend;

pub struct App<B> {
    pub ui: UI<B>,
    pub system: System,
    pub session: Session,
}

impl<B> App<B>
where
    B: Backend,
{
    pub fn new() -> Self {
        Self {
            ui: UI::new(),
            system: System::new(),
            session: Session::new(),
        }
    }
}
