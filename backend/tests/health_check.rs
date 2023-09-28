use hyper::Uri;

#[tokio::test]
async fn health_check_works(){

    let client = hyper::Client::new();

    let response = client
        .get(Uri::from_static("http://127.0.0.1:8080/health_check"))
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());    
}
