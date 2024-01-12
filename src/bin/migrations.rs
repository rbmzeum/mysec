use openssl::ssl::{SslConnector, SslMethod, SslFiletype, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use std::sync::Arc;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use tokio::fs::try_exists;

use secsak::modules::verify::Store as VirifyStore;

const MS_STATUS_APPLIED: u8 = 0;
const MS_STATUS_FAILED: u8 = 1;

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

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "rollback" => {
                println!("MIGRATION DOWN");

                let mut migration_path = current_dir.clone();
                migration_path.push("database/migrations");

                let mut migrations = fs::read_dir(&migration_path)?.map(|v| v.unwrap().path()).collect::<Vec<_>>();
                migrations.sort();
                migrations.reverse();
                for path in migrations {
                    let mut status_path = path.clone();
                    status_path.push("status");
                    if try_exists(&status_path).await? {
                        let mut file = File::open(&status_path)?;
                        let mut status = [0; 1];
                        file.read_exact(&mut status)?;
                        if status[0] != MS_STATUS_APPLIED {
                            continue;
                        }
                    }

                    let mut down_path = path.clone();
                    down_path.push("down.sql");

                    let down_contents = fs::read_to_string(down_path)
                        .expect("Should have been able to read the file down.sql");

                    println!("Rollback SQL:\n{:#?}", &down_contents);
                    client.batch_execute(down_contents.as_str()).await?;

                    fs::remove_file(&status_path)?;
                }
            },
            _ => {
                println!("UNKNOWN PARAMENTER");
            },
        }
    } else {
        println!("MIGRATION UP");

        let mut migration_path = current_dir.clone();
        migration_path.push("database/migrations");

        let mut migrations = fs::read_dir(&migration_path)?.map(|v| v.unwrap().path()).collect::<Vec<_>>();
        migrations.sort();
        for path in migrations {
            let mut status_path = path.clone();
            status_path.push("status");
            if try_exists(&status_path).await? {
                let mut file = File::open(&status_path)?;
                let mut status = [0; 1];
                file.read_exact(&mut status)?;
                if status[0] == MS_STATUS_APPLIED {
                    continue;
                }
            }

            let mut up_path = path.clone();
            up_path.push("up.sql");

            let up_contents = fs::read_to_string(up_path)
                .expect("Should have been able to read the file up.sql");

            println!("SQL:\n{:#?}", &up_contents);
            client.batch_execute(up_contents.as_str()).await?;

            let mut file = File::create(&status_path)?;
            file.write_all(&[MS_STATUS_APPLIED])?;
        }
    }

    Ok(())
}