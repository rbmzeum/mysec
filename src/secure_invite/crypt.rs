use x448::{
    Secret,
    PublicKey,
};

use rand_core::{
    OsRng,
};

use base64::{Engine as _, engine::general_purpose};

use aes::Aes256;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

use crate::secure_invite::keypair::KeyPair;

pub struct Crypt {}

impl Crypt {

    pub fn make_keypair() -> KeyPair
    {
        let mut csprng = OsRng::default();
        let secret = Secret::new(&mut csprng);
        let public_key = PublicKey::from(&secret);

        KeyPair {
            secret: secret,
            public: public_key,
        }
    }

    pub fn encrypt(shared_key_string: String, data: &[u8]) -> (usize, Vec<u8>)
    {
        let shared_key_bytes_restored = general_purpose::STANDARD_NO_PAD.decode(shared_key_string).unwrap();
        let key = GenericArray::from_slice(shared_key_bytes_restored[..32].as_ref());
        let encrypt_cipher = Aes256::new(&key);

        // TODO: добавлять к data данные случайного размера для сокрытия размера исходных данных

        let message_encrypted_chunks: Vec<_> = data.chunks(16).map(|chunk| {
            let mut bytes = [0_u8; 16];
            bytes[..chunk.len()].copy_from_slice(&chunk);
            let mut block = GenericArray::clone_from_slice(&bytes);
            encrypt_cipher.encrypt_block(&mut block);
            block
        }).collect();
        let encrypted_data = message_encrypted_chunks.concat();
        (encrypted_data.len()-data.len(), encrypted_data)
    }

    pub fn decrypt(shared_key_string: String, data: &[u8]) -> Vec<u8>
    {
        let shared_key_bytes_restored = general_purpose::STANDARD_NO_PAD.decode(shared_key_string).unwrap();
        let key = GenericArray::from_slice(shared_key_bytes_restored[..32].as_ref());
        let decrypt_cipher = Aes256::new(&key);

        let mut decrypt_data = vec![];
        for block in data.chunks(16) {
            let mut b = GenericArray::clone_from_slice(block);
            decrypt_cipher.decrypt_block(&mut b);
            decrypt_data = [decrypt_data, b.to_vec()].concat();
        }
        // TODO: вычленять из decrypt_data данные добавленные перед шифрованием для сокрытия размера исходных данных
        decrypt_data
    }

}
