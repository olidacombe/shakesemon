use shakesemon::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO get bind address and port from environment vars
    let address = "127.0.0.1";
    let port = "8000";
    // Bubble up the io::Error if we failed to bind the address
    let listener =
        TcpListener::bind(format!("{}:{}", address, port)).expect("Failed to bind random port");
    run(listener)?.await
}
