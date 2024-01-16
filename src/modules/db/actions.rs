use std::sync::{Arc, Mutex};
use crate::modules::db::state::State;
// use crate::modules::db::mutations::Mutations;

pub struct Actions {
    state: Arc<Mutex<State>>,
}

impl Actions {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Actions { state }
    }
}