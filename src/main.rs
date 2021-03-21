use shakesemon::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address: String =
        std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:0".to_owned());
    // Bubble up the io::Error if we failed to bind the address
    let listener = TcpListener::bind(&bind_address)
        .unwrap_or_else(|_| panic!("Failed to bind {}", &bind_address));
    run(listener)?.await
}
