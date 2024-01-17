use std::sync::{Arc, Mutex};
use crate::modules::db::state::State;
use tokio_postgres::Client;

pub struct Getters {
    state: Arc<Mutex<State>>,
}

impl Getters {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Getters { state }
    }

    pub fn get_client(&self) -> Arc<Mutex<Client>> {
        let state = self.state.lock().unwrap(); // FIXME: переписать код с обработкой ошибки вместо использования unwrap
        state.client.clone()
    }
}