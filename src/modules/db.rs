pub mod getters;
pub mod mutations;
pub mod actions;
pub mod state;

use std::sync::{Arc, Mutex};
use state::State;
use getters::Getters;
use mutations::Mutations;
use actions::Actions;

pub struct Store {
    pub state: Arc<Mutex<State>>,
    pub getters: Getters,
    pub mutations: Mutations,
    pub actions: Actions,
}

impl Store {
    pub async fn new(dsn: &str) -> Store {
        let initial_state = State::new(dsn).await;
        let state = Arc::new(Mutex::new(initial_state.unwrap()));
        let getters = Getters::new(state.clone());
        let mutations = Mutations::new();
        let actions = Actions::new(state.clone());

        Store {
            state,
            getters,
            mutations,
            actions,
        }
    }
}