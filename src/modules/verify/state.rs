use std::collections::BTreeSet;

pub struct State {
    pub hashes: BTreeSet<String>,
    pub verified: bool,
}

impl State {
    pub fn new() -> State {
        State {
            hashes: BTreeSet::new(),
            verified: false,
        }
    }
}