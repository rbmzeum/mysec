use crate::modules::db::state::State;
use tokio_postgres::Client;
use std::sync::{Arc, Mutex};

pub struct Mutations;

impl Mutations {
    pub fn new() -> Self {
        Mutations
    }

    pub fn set_client(state: &mut State, client: Client) {
        state.client = Arc::new(Mutex::new(client))
    }
}