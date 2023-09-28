#[tokio::test]
async fn signup_returns_a_200_for_valid_form_data() {
    let client = hyper::Client::new();

    let body = hyper::Body::from("username=zoro juro");

    let request = hyper::Request::builder()
        .method(hyper::Method::POST)
        .uri("http://127.0.0.1:8080/users")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .expect("Failed to create request");

    let send_request = client
        .request(request)
        .await
        .expect("Failed to send request");

    assert_eq!(send_request.status(), hyper::StatusCode::OK);
}

#[tokio::test]
async fn signup_returns_a_400_when_username_is_missing(){
    let client = hyper::Client::new();

    let body = hyper::Body::empty();

    let request = hyper::Request::builder()
        .method(hyper::Method::POST)
        .uri("http://127.0.0.1:8080/users")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .expect("Failed to create request");

    let send_request = client
        .request(request)
        .await
        .expect("Failed to send request");

    assert_eq!(send_request.status(), hyper::StatusCode::BAD_REQUEST);
}