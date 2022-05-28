
#[tokio::test]
async fn health_check_test() {
    spawn_app();
    let client = reqwest::Client::new();

    let response = client.get("http://127.0.0.1:8080/health_check").send().await.expect("Request failed");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = newsletter_backend_example::run().expect("Failed to spawn the app");
    let _ = tokio::spawn(server);
}
