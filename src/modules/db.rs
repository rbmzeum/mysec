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