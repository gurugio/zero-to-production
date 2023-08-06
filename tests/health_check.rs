#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", addr))
        .send()
        .await
        .expect("failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let addr = listener.local_addr().unwrap().ip();

    let server = zero_to_production::run(listener).expect("failed to bind server");
    let _ = tokio::spawn(server);
    format!("http://{}:{}", addr, port)
}
