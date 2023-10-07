use hyper::Uri;

#[tokio::test]
async fn sse_returns_stored_data(){

    let client = hyper::Client::new();


    let response = client
        .get(Uri::from_static("http://127.0.0.1:8080/sse"))
        .await
        .expect("failed to send request");

    assert_eq!(response.status(), hyper::StatusCode::OK);
}