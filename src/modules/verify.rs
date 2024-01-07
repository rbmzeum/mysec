pub mod getters;
pub mod mutations;
pub mod actions;

use std::collections::BTreeSet;

pub struct State<T> {
    pub hashes: BTreeSet<String>, // TODO: вместо String использовать [u8, 32] (во время отладки используется String)
    pub value: T,
}

pub enum Mutation {
    Verified,
    Failed,
}

pub enum Action {
    FetchHashes,
    Verify,
}

impl<T> State<T> where T: Copy {
    pub fn get_value(&self) -> T {
        self.value
    }

    pub fn get_hashes(&self) -> &BTreeSet<String> {
        &self.hashes
    }
}

impl State<bool> {
    pub fn commit(&mut self, mutation: Mutation) {
        match mutation {
            Mutation::Verified => self.value = true,
            Mutation::Failed => self.value = false,
        }
    }

    pub async fn dispatch(&mut self, action: Action/*, arguments: Vec<Box<dyn Argument>>*/) {
        match action {
            Action::FetchHashes => {
                self.hashes = self.fetch_available_hashes_from_blockchain(/* TODO: Client ID or License Key */).await;
            }
            Action::Verify => {
                // TODO: выполнить проверку наличия хеша из параметра в списке разрешённых хешей
            }
            _ => {}
        }
    }

    async fn fetch_available_hashes_from_blockchain(&self) -> BTreeSet<String> {
        let mut hashes = BTreeSet::new();

        // TODO: загружать из блокчейна по ID клиента (по лицензионному ключу)
        hashes.insert(String::from("4sXyRGOudf3KJ+26VPVi9bsRURfDPbbNFrho2IEa678"));
        hashes.insert(String::from("QNouKo/2uuV0WIIFMllJeqz2GeR6pdo6KAhtmmT6dI8"));

        hashes
    }
}