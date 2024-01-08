use std::sync::{Arc, Mutex};
use std::collections::BTreeSet;
use crate::modules::verify::state::State;

pub struct Getters {
    state: Arc<Mutex<State>>,
}

impl Getters {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Getters { state }
    }

    pub fn get_verified(&self) -> bool {
        let state = self.state.lock().unwrap(); // FIXME: переписать код с обработкой ошибки вместо использования unwrap
        state.verified.clone()
    }

    pub fn get_hashes(&self) -> BTreeSet<String> {
        let state = self.state.lock().unwrap(); // FIXME: переписать код с обработкой ошибки вместо использования unwrap
        state.hashes.clone()
    }
}