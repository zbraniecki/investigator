use crate::views;
use tui::widgets::TableState;

pub struct UIState {
    pub active_menu_idx: usize,
    pub view_states: Vec<TableState>,
}

impl UIState {
    pub fn new(views: usize) -> Self {
        Self {
            active_menu_idx: 0,
            view_states: vec![TableState::default(); views],
        }
    }
}

pub struct UI {
    pub views: Vec<views::View>,
    pub state: UIState,
}

impl UI {
    pub fn new() -> Self {
        let views = views::get_all();
        let len = views.len();
        Self {
            views,
            state: UIState::new(len),
        }
    }

    pub fn get_active_view(&self) -> &views::View {
        let idx = self.state.active_menu_idx;
        self.views.get(idx).unwrap()
    }

    pub fn get_active_view_state(&mut self) -> &mut TableState {
        let idx = self.state.active_menu_idx;
        &mut self.state.view_states[idx]
    }
}
