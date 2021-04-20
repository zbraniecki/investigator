pub mod event;
pub mod widgets;

use crate::{
    app::App,
    ui::{
        event::HandleEventResult,
        widgets::{Input, Menu},
    },
    views,
};
use std::{cell::RefCell, ops::Deref};
use termion::event::Key;
use tui::backend::Backend;

pub struct UIState {
    active_view_idx: usize,
}

impl UIState {
    pub fn new() -> Self {
        Self { active_view_idx: 0 }
    }
}

pub struct UI<B> {
    pub views: Vec<Box<dyn views::ViewType<B>>>,
    pub menu: Menu,
    pub input: Input,
    pub state: RefCell<UIState>,
}

impl<B> UI<B>
where
    B: Backend,
{
    pub fn new() -> Self {
        let views = views::get_all();
        Self {
            views,
            menu: Menu::new(),
            input: Input::new(),
            state: RefCell::new(UIState::new()),
        }
    }

    pub fn get_active_view_idx(&self) -> usize {
        let state = self.state.borrow();
        let idx = state.active_view_idx;
        idx
    }

    pub fn get_active_view(&self) -> &dyn views::ViewType<B> {
        let state = self.state.borrow();
        let idx = state.active_view_idx;
        let view: &Box<dyn views::ViewType<_>> = self.views.get(idx).unwrap();
        view.deref()
    }

    pub fn get_view_names(&self) -> Vec<String> {
        self.views
            .iter()
            .map(|view| view.get_name().to_string())
            .collect()
    }

    pub fn set_view(&self, name: &str, app: &App<B>) {
        let idx = self
            .views
            .iter()
            .position(|view| view.get_name() == name)
            .unwrap();
        let mut state = self.state.borrow_mut();
        if state.active_view_idx != idx {
            let old_view = &self.views[state.active_view_idx];
            old_view.before_hide(app);
            let old_view = &self.views[idx];
            old_view.before_display(app);
        }
        state.active_view_idx = idx;
    }

    pub fn handle_event(&self, key: &Key, app: &App<B>) -> HandleEventResult {
        if app.ui.input.is_active() {
            match app.ui.input.handle_event(key, app) {
                HandleEventResult::Handled => {
                    return HandleEventResult::Handled;
                }
                _ => {}
            }
        }

        let active_view = self.get_active_view();
        match active_view.handle_event(key, app) {
            HandleEventResult::Bubbled => self.menu.handle_event(key, app),
            result @ _ => result,
        }
    }
}
