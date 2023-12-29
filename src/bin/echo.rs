use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::thread::spawn;
use tungstenite::accept;
use openssl::ssl::{
    SslStream,
    SslMethod,
    SslVerifyMode,
    SslAcceptor,
    SslFiletype
};
use std::num::ParseIntError;

/// A WebSocket echo server
fn main () -> Result<(), ParseIntError> {
    let current_dir = std::env::current_dir().expect("failed to read current directory");
    let mut cert = current_dir.clone();
    let mut key = current_dir.clone();
    let mut ca = current_dir.clone();
    cert.push("ssl/certs/wssserver.local.crt");
    key.push("ssl/certs/wssserver.local.key");
    ca.push("ssl/certs/myCA.pem");
    println!("{:#?}\n{:#?}\n{:#?}", &current_dir, &cert, &key);

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls_server()).unwrap();
    acceptor.set_private_key_file(key, SslFiletype::PEM).unwrap();
    // acceptor.set_certificate_file(cert, SslFiletype::PEM).unwrap();
    acceptor.set_certificate_chain_file(cert).unwrap();
    acceptor.set_ca_file(ca).unwrap();
    acceptor.check_private_key().unwrap();
    acceptor.set_verify(SslVerifyMode::NONE); // DEBUG !!!! в продакшне заменить NONE на PEER и вынести в конфиг
    // accept all certificates, we'll do our own validation on them
    acceptor.set_verify_callback(SslVerifyMode::NONE, |_, _| true); // DEBUG !!!! в продакшне заменить NONE на PEER и вынести в конфиг
    let acceptor = Arc::new(acceptor.build());

    fn handle_client(stream: SslStream<TcpStream>) {
        let mut websocket: tungstenite::WebSocket<SslStream<TcpStream>> = accept(stream).unwrap();
        loop {
            let msg = websocket.read().unwrap();

            // We do not want to send back ping/pong messages.
            if msg.is_binary() || msg.is_text() {
                websocket.send(msg).unwrap();
            }
        }
    }

    let server = TcpListener::bind("127.0.0.1:444").unwrap();
    for stream in server.incoming() {
        let acceptor = acceptor.clone();
        spawn (move || {
            let mut sslStream = acceptor.accept(stream.unwrap()).unwrap();
            handle_client(sslStream);
        });
    }

    Ok(())
}