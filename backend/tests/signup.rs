use backend::config::Config;
use sqlx::postgres::PgPoolOptions;


#[tokio::test]
async fn signup_returns_a_200_for_valid_form_data() {
    let config: Config = Config::new();

    let db_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to db");

    let client = hyper::Client::new();

    let body = hyper::Body::from("username=zoro juro");

    let request = hyper::Request::builder()
        .method(hyper::Method::POST)
        .uri("http://127.0.0.1:8080/users")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .expect("Failed to create request");

    let _response = client
        .request(request)
        .await
        .expect("Failed to send request");

    //assert_eq!(response.status(), hyper::StatusCode::OK);

    let saved = sqlx::query!(
        "select username, referral_code from users",
        )
        .fetch_one(&db_connection)
        .await
        .expect("Failed to fetch saved data");

    assert_eq!(saved.username, "zorojuro");
    //assert_eq!(saved.referral_code, "");

}




//#[tokio::test]
// async fn signup_returns_a_400_when_username_is_missing(){
//     let client = hyper::Client::new();

//     let body = hyper::Body::empty();

//     let request = hyper::Request::builder()
//         .method(hyper::Method::POST)
//         .uri("http://127.0.0.1:8080/users")
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .expect("Failed to create request");

//     let response = client
//         .request(request)
//         .await
//         .expect("Failed to send request");

//     assert_eq!(response.status(), hyper::StatusCode::BAD_REQUEST);
// }