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
    pub fn new() -> Store {
        let initial_state = State::new();
        let state = Arc::new(Mutex::new(initial_state));
        let getters = Getters::new(Arc::clone(&state));
        let mutations = Mutations::new();
        let actions = Actions::new(Arc::clone(&state));

        Store {
            state,
            getters,
            mutations,
            actions,
        }
    }
}