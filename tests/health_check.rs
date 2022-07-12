use std::net::TcpListener;

#[tokio::test]
async fn health_check_test() {
    let base_url = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &base_url))
        .send()
        .await
        .expect("Request failed");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind a port");
    let port = listener.local_addr().unwrap().port();
    let server = newsletter_backend_example::run(listener).expect("Failed to spawn the app");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
