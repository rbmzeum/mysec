use x448::{
    Secret,
    PublicKey,
};

use base64::{Engine as _, engine::general_purpose};

pub struct KeyPair {
    pub secret: Secret,
    pub public: PublicKey,
}

impl KeyPair {
    pub fn make_shared_key(&self, key: &PublicKey) -> String
    {
        let shared_secret = self.secret.as_diffie_hellman(key).unwrap();
        let shared_key_bytes = shared_secret.as_bytes().to_vec();
        let shared_key_string = general_purpose::STANDARD_NO_PAD.encode(shared_key_bytes);
        shared_key_string
    }
}