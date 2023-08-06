use zero_to_production::run;

use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let addr = listener.local_addr().unwrap().ip();
    println!("listen addr={}, port={}", addr, port);

    run(listener)?.await
}
