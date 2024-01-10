use openssl::ssl::{SslConnector, SslMethod, SslFiletype, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use openssl::sha::sha256;
use base64::{Engine as _, engine::general_purpose};
use std::sync::{Arc, Mutex};

use secsak::modules::verify::Store as VirifyStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let current_dir = std::env::current_dir().expect("failed to read current directory");
    let mut cert = current_dir.clone();
    let mut key = current_dir.clone();
    let mut ca = current_dir.clone();
    cert.push("ssl/db/postgresql.crt");
    key.push("ssl/db/postgresql.key");
    ca.push("ssl/db/myCA.crt");
    println!("{:#?}\n{:#?}\n{:#?}", &current_dir, &cert, &key);

    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_private_key_file(key, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(cert).unwrap();
    builder.set_ca_file(ca).unwrap();
    builder.check_private_key().unwrap();
    builder.set_verify(SslVerifyMode::NONE); // DEBUG !!!! в продакшне заменить NONE на PEER и вынести в конфиг
    // accept all certificates, we'll do our own validation on them

    let verify_store = VirifyStore::new();
    verify_store.actions.init_hashes().await;

    // TODO: вынести callback в middleware
    builder.set_verify_callback(SslVerifyMode::NONE, move |_success, ctx| {
        if let Some(cert) = ctx.current_cert() {
            if let Ok(pkey) = cert.public_key() {
                if let Ok(pem) = pkey.public_key_to_pem() {
                    let hash = sha256(&pem);
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
    }); // DEBUG !!!! в продакшне заменить NONE на PEER и вынести в конфиг

    let connector = MakeTlsConnector::new(builder.build());

    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres sslmode=require dbname=mysec",
        connector,
    ).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )
    ").await?;

    client.batch_execute("
        DROP TABLE person;
    ").await?;

    Ok(())
}