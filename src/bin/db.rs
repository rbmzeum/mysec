use openssl::ssl::{SslConnector, SslMethod, SslFiletype, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use std::sync::Arc;
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
    builder.set_verify(SslVerifyMode::PEER);
    // accept all certificates, we'll do our own validation on them

    let verify_store = VirifyStore::new();
    verify_store.actions.init_hashes().await;
    let vs = Arc::new(verify_store);
    builder.set_verify_callback(SslVerifyMode::PEER, vs.getters.get_verify_callback(vs.clone()));

    let connector = MakeTlsConnector::new(builder.build());

    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=uqlmftqj sslmode=require dbname=mysec",
        connector,
    ).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let id: i64 = 1;
    let qr = client.query("SELECT * FROM persons WHERE id = $1", &[&id]).await?;
    for row in qr {
        let data: Vec<u8> = row.get("data");
        println!("data: {:#?}", &data);
    }

    let data = vec![0xccu8];
    let qr = client.query("SELECT * FROM persons WHERE data = $1", &[&data]).await?;
    for row in qr {
        let id: i64 = row.get("id");
        let data: Vec<u8> = row.get("data");
        println!("id, data: {:#?}, {:#?}", &id, &data);
    }

    let data = vec![0xffu8];
    client.query("UPDATE persons SET data = $1 WHERE id = $2", &[&data, &id]).await?;

    Ok(())
}