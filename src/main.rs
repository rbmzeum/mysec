mod secure_invite;
pub use crate::secure_invite::{
    keypair::KeyPair,
    crypt::Crypt,
};

fn main() {
    let alice_keypair = Crypt::make_keypair();
    let bob_keypair = Crypt::make_keypair();

    // Боб передаёт Алисе свой публичный ключ, по которому Алиса создаёт общий ключ
    let alices_shared_key_string = alice_keypair.make_shared_key(&bob_keypair.public);
    println!("Alices shared key: {:#?}", &alices_shared_key_string);

    // Алиса передаёт Бобу свой публичный ключ, по которому Боб создаёт общий ключ
    let bobs_shared_key_string = bob_keypair.make_shared_key(&alice_keypair.public);
    println!("Bobs shared key: {:#?}", &bobs_shared_key_string);

    let message = String::from("Secret phrase");
    let message_bytes = message.as_bytes();

    // Алиса шифрует сообщение и передаёт Бобу length и message_encrypted
    let (length, message_encrypted) = Crypt::encrypt(alices_shared_key_string, message_bytes);
    println!("Message encrypted {:#?}", &message_encrypted);

    // Боб расшифровывает сообщение
    let decrypted_message = Crypt::decrypt(bobs_shared_key_string, &message_encrypted);
    println!("Decrypted message: {:#?}", std::str::from_utf8(decrypted_message[..decrypted_message.len()-length].as_ref()).unwrap());
}
