use warp::Filter;

#[tokio::main]
async fn main() {
    let current_dir = std::env::current_dir().expect("failed to read current directory");
    let mut public_dir = current_dir.clone();
    let mut certs_path = current_dir.clone();
    let mut key_path = current_dir.clone();
    public_dir.push("public");
    certs_path.push("ssl/wss/wssserver.local.crt");
    key_path.push("ssl/wss/wssserver.local.key");
    println!("{:#?}\n{:#?}\n{:#?}", &current_dir, &certs_path, &key_path);
    let routes = warp::get().and(warp::fs::dir(public_dir));
    warp::serve(routes)
        .tls()
        .cert_path(certs_path)
        .key_path(key_path)
        .run(([127, 0, 0, 1], 443)).await;
}