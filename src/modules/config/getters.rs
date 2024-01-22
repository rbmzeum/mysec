use std::sync::{Arc, Mutex};
use crate::modules::config::state::State;

pub struct Getters {
    state: Arc<Mutex<State>>,
}

impl Getters {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Getters { state }
    }

    pub fn get_dsn(&self) -> String {
        let state = self.state.lock().unwrap(); // FIXME: переписать код с обработкой ошибки вместо использования unwrap
        let btm = &state.tree_map;
        match btm.get("dsn") {
            Some(v) => v.clone(),
            None => String::from(""),
        }
    }

    pub fn get_dsn_root(&self) -> String {
        let state = self.state.lock().unwrap(); // FIXME: переписать код с обработкой ошибки вместо использования unwrap
        let btm = &state.tree_map;
        match btm.get("dsn_root") {
            Some(v) => v.clone(),
            None => String::from(""),
        }
    }
}