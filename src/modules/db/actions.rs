use std::sync::{Arc, Mutex};
use crate::modules::db::state::State;
// use crate::modules::db::mutations::Mutations;
use tokio_postgres::types::ToSql;

pub struct Actions {
    state: Arc<Mutex<State>>,
}

impl Actions {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Actions { state }
    }

    pub async fn batch_execute(&self, sql: &str) -> Result<(), tokio_postgres::Error> {
        let state = self.state.lock().unwrap();
        let result = state.client.lock().unwrap().batch_execute(sql).await;
        result
    }

    pub async fn query(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error> {
        let state = self.state.lock().unwrap();
        let result = state.client.lock().unwrap().query(sql, params).await;
        result
    }

    pub async fn execute(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, tokio_postgres::Error> {
        let state = self.state.lock().unwrap();
        let rows_updated = state.client.lock().unwrap().execute(sql, params).await;
        rows_updated
    }
}