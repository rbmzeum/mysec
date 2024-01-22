use std::sync::{Arc, Mutex};
use crate::modules::config::state::State;

pub struct Actions {
    state: Arc<Mutex<State>>,
}

impl Actions {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Actions { state }
    }
}