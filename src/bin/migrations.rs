use openssl::ssl::{SslConnector, SslMethod, SslFiletype, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use std::sync::Arc;
use std::env;
use std::fs;

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
    let vs = Arc::new(verify_store);
    builder.set_verify_callback(SslVerifyMode::NONE, vs.getters.get_verify_callback(vs.clone())); // DEBUG !!!! в продакшне заменить NONE на PEER и вынести в конфиг

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

    // 
    // for argument in env::args() {
    //     println!("{argument}");
    // }
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "rollback" => {
                println!("MIGRATION DOWN");
            },
            _ => {
                println!("UNKNOWN PARAMENTER");
            },
        }
    } else {
        println!("MIGRATION UP");

        let mut migration_path = current_dir.clone();
        migration_path.push("database/migrations");

        // TODO: отсортировать
        for entry in fs::read_dir(migration_path)? {
            let entry = entry?;
            let path = entry.path();

            let mut up_path = path.clone();
            up_path.push("up.sql");

            // let mut down_path = path.clone();
            // down_path.push("down.sql");

            let up_contents = fs::read_to_string(up_path)
                .expect("Should have been able to read the file up.sql");
            // let down_contents = fs::read_to_string(down_path)
            //     .expect("Should have been able to read the file down.sql");

            println!("SQL:\n{:#?}", &up_contents);
            client.batch_execute(up_contents.as_str()).await?;
            // TODO: создать файл с информацией о выполнении up.sql (для того, чтобы при последующих запусках проигнорировать выполнение данной миграции)
        }
    }

    Ok(())
}