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

    pub async fn batch_execute(&self, sql: &str) -> Result<(), tokio_postgres::Error> {
        let mut state = self.state.lock().unwrap();
        let result = state.client.lock().unwrap().batch_execute(sql).await;
        result
    }
}