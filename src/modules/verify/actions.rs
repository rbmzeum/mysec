use std::sync::{Arc, Mutex};
use std::collections::BTreeSet;
use crate::modules::verify::state::State;
use crate::modules::verify::mutations::{
    Mutations,
    Mutation,
};

pub struct Actions {
    state: Arc<Mutex<State>>,
}

impl Actions {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Actions { state }
    }

    pub async fn init_hashes(&self) {
        let mut state = self.state.lock().unwrap();
        let hashes = self.fetch_available_hashes_from_blockchain().await;
        Mutations::set_hashes(&mut state, hashes);
    }

    pub async fn verify(&self, hash: String) {
        let mut state = self.state.lock().unwrap();
        let is_verified = state.hashes.contains(&hash);
        match is_verified {
            true => Mutations::set_verified(&mut state, Mutation::Verified),
            false => Mutations::set_verified(&mut state, Mutation::Failed),
        }
    }

    async fn fetch_available_hashes_from_blockchain(&self) -> BTreeSet<String> {
        let mut hashes = BTreeSet::new();

        // TODO: загружать из блокчейна по ID клиента (по лицензионному ключу)
        // for db client
        hashes.insert(String::from("ASyYNf22TBA/YEpT0k24YGspA8QZZYnnjeXomDFmPkoiSHutI7b8t4hOAq0IznS+"));
        hashes.insert(String::from("uMSp6Cn/eQjCcCMzdrv6fyGOuiT9iBv9alJeKsQjUZXQzV3f3FUx1aZ/m2/kw+qq"));

        // for websocket server

        hashes
    }
}