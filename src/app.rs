use crate::state::State;
use crate::system::{System, SystemData};
use crate::ui::UI;

pub struct App {
    pub ui: UI,
    pub system: System,
    pub state: State,
}

impl App {
    pub fn new() -> Self {
        Self {
            ui: UI::new(),
            system: System::new(),
            state: State::new(),
        }
    }

    pub fn get_view_len(&self, idx: usize) -> usize {
        match self.ui.views[idx].name.as_str() {
            "Transactions" => self.state.transactions.len(),
            "Wallets" => self.state.wallets.len(),
            "Exchanges" => self.state.exchanges.len(),
            "Portfolio" => self.state.portfolios.len(),
            _ => 0,
        }
    }

    pub fn get_system_data(&self) -> &SystemData {
        &self.system.data
    }

    pub fn next(&mut self) {
        let idx = self.ui.state.active_menu_idx;
        let len = self.get_view_len(idx);

        let i = match self.ui.state.view_states[idx].selected() {
            Some(i) => {
                if i >= len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.ui.state.view_states[idx].select(Some(i));
    }

    pub fn previous(&mut self) {
        let idx = self.ui.state.active_menu_idx;
        let len = self.get_view_len(idx);

        let i = match self.ui.state.view_states[idx].selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.ui.state.view_states[idx].select(Some(i));
    }
}
