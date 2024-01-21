use std::sync::{Arc, Mutex};
use std::collections::BTreeSet;
use crate::modules::verify::state::State;
use crate::modules::verify::Store;
use openssl::x509::X509StoreContextRef;
use sha3::{Digest, Sha3_384};
use base64::{Engine as _, engine::general_purpose};

pub struct Getters {
    state: Arc<Mutex<State>>,
}

impl Getters {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Getters { state }
    }

    pub fn get_verified(&self) -> bool {
        let state = self.state.lock().unwrap(); // FIXME: переписать код с обработкой ошибки вместо использования unwrap
        state.verified.clone()
    }

    pub fn get_hashes(&self) -> BTreeSet<String> {
        let state = self.state.lock().unwrap(); // FIXME: переписать код с обработкой ошибки вместо использования unwrap
        state.hashes.clone()
    }

    pub fn get_verify_callback(&self, verify_store: Arc<Store>) -> impl Fn(bool, &mut X509StoreContextRef) -> bool {
        move |_success, ctx: &mut X509StoreContextRef| {
            if let Some(cert) = ctx.current_cert() {
                if let Ok(pkey) = cert.public_key() {
                    if let Ok(pem) = pkey.public_key_to_pem() {
                        let mut hasher = Sha3_384::new();
                        hasher.update(&pem);
                        let hash = hasher.finalize();
                        let vs = Arc::new(Mutex::new(&verify_store));
                        let task = async move {
                            let verify_store = vs.lock().unwrap();
                            verify_store.actions.verify(general_purpose::STANDARD_NO_PAD.encode(&hash)).await;
                        };
                        futures::executor::block_on(task);
                        let res = verify_store.getters.get_verified();
                        println!("Hash: {:#?} {:#?}", general_purpose::STANDARD_NO_PAD.encode(&hash), &res);
                        return res; // return hash.trim().eq_ignore_ascii_case(&cmp_hash) // cmp_hash можно хранить в блокчейне
                    }
                }
            }
            false
        }
    }
}