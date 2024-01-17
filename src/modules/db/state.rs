use openssl::ssl::{SslConnector, SslMethod, SslFiletype, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use std::sync::{Arc, Mutex};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use tokio::fs::try_exists;

use crate::modules::verify::Store as VirifyStore;

use tokio_postgres::Client;

pub struct State {
    pub client: Arc<Mutex<Client>>,
}

impl State {
    pub async fn new(cdn: &str) -> Result<State, Box<dyn std::error::Error>> {
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
        let vs = Arc::new(verify_store);
        builder.set_verify_callback(SslVerifyMode::NONE, vs.getters.get_verify_callback(vs.clone())); // DEBUG !!!! в продакшне заменить NONE на PEER и вынести в конфиг

        let connector = MakeTlsConnector::new(builder.build());

        let (client, connection) = tokio_postgres::connect(
            cdn,
            connector,
        ).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(State {
            client: Arc::new(Mutex::new(client)),
        })
    }
}