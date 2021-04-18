use crate::views;
use std::ops::Deref;
use tui::{backend::Backend, widgets::TableState};

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

pub struct UI<B> {
    pub views: Vec<Box<dyn views::ViewType<B>>>,
    pub state: UIState,
}

impl<B> UI<B>
where
    B: Backend,
{
    pub fn new() -> Self {
        let views = views::get_all();
        let len = views.len();
        Self {
            views,
            state: UIState::new(len),
        }
    }

    pub fn get_active_view(&self) -> &dyn views::ViewType<B> {
        let idx = self.state.active_menu_idx;
        let view: &Box<dyn views::ViewType<_>> = self.views.get(idx).unwrap();
        view.deref()
    }
}
