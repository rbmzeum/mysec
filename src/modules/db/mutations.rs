use crate::modules::db::state::State;
use tokio_postgres::Client;

pub struct Mutations;

impl Mutations {
    pub fn new() -> Self {
        Mutations
    }

    pub fn set_client(state: &mut State, client: Client) {
        state.client = client
    }
}