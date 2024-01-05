use openssl::ssl::{SslConnector, SslMethod, SslFiletype, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;

fn main() -> Result<(), Box<dyn std::error::Error>> {

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
    builder.set_verify_callback(SslVerifyMode::NONE, |_, _| true); // DEBUG !!!! в продакшне заменить NONE на PEER и вынести в конфиг и возвращать не true, а результат проверки: true or false
    let connector = MakeTlsConnector::new(builder.build());

    let mut client = postgres::Client::connect(
        "host=localhost user=postgres sslmode=require dbname=mysec",
        connector,
    )?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )
    ")?;

    client.batch_execute("
        DROP TABLE person;
    ")?;

    Ok(())
}