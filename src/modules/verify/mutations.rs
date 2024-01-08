use crate::modules::verify::state::State;
use std::collections::BTreeSet;

pub enum Mutation {
    Verified,
    Failed,
}

pub struct Mutations;

impl Mutations {
    pub fn new() -> Self {
        Mutations
    }

    pub fn set_verified(state: &mut State, mutation: Mutation) {
        match mutation {
            Mutation::Verified => state.verified = true,
            Mutation::Failed => state.verified = false,
        }
    }

    pub fn set_hashes(state: &mut State, hashes: BTreeSet<String>) {
        state.hashes = hashes
    }
}