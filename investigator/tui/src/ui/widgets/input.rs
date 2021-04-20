use crate::{app::App, ui::event::HandleEventResult};
use std::cell::RefCell;
use termion::event::Key;

pub struct InputState {
    value: Option<f64>,
    active: bool,
    fract: bool,
    interacted: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            value: None,
            active: false,
            fract: false,
            interacted: false,
        }
    }
}

pub struct Input {
    state: RefCell<InputState>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(InputState::new()),
        }
    }

    pub fn set_value(&self, value: Option<f64>) {
        let mut state = self.state.borrow_mut();
        state.value = value;
    }

    pub fn value(&self) -> Option<f64> {
        let state = self.state.borrow();
        state.value
    }

    pub fn activate(&self) {
        let mut state = self.state.borrow_mut();
        state.active = true;
        state.fract = false;
        state.interacted = false;
    }

    pub fn is_active(&self) -> bool {
        let state = self.state.borrow();
        state.active
    }

    pub fn interacted(&self) -> bool {
        let state = self.state.borrow();
        state.interacted
    }

    pub fn deactivate(&self) {
        let mut state = self.state.borrow_mut();
        state.active = false;
    }

    pub fn handle_event<B>(&self, key: &Key, app: &App<B>) -> HandleEventResult {
        match key {
            Key::Char(ch) => {
                if ch.is_ascii_digit() {
                    let digit = ch.to_digit(10).unwrap();

                    let mut state = self.state.borrow_mut();
                    state.interacted = true;
                    let is_fract = state.fract;
                    if let Some(value) = &mut state.value {
                        if is_fract {
                            let mut s = value.to_string();
                            if !s.contains('.') {
                                s.push('.');
                            }
                            s.push(*ch);
                            let result: f64 = s.parse().unwrap();
                            state.value = Some(result);
                        } else {
                            let int_part = unsafe { value.to_int_unchecked::<usize>() };
                            let mut s = int_part.to_string();
                            s.push(*ch);
                            let result: f64 = s.parse().unwrap();
                            state.value = Some(result);
                        }
                    } else {
                        if is_fract {
                            state.value = Some((digit as f64) / 10.0);
                        } else {
                            state.value = Some(digit as f64);
                        }
                    }
                    HandleEventResult::Handled
                } else if *ch == '.' {
                    let mut state = self.state.borrow_mut();
                    state.fract = true;
                    HandleEventResult::Handled
                } else {
                    HandleEventResult::Bubbled
                }
            }
            Key::Backspace => {
                let mut state = self.state.borrow_mut();
                state.value = None;
                state.fract = false;
                state.interacted = true;
                HandleEventResult::Handled
            }
            _ => HandleEventResult::Bubbled,
        }
    }
}
