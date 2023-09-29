use backend::config::Config;
use sqlx::postgres::PgPoolOptions;


//make sure to use database transactions in tests
//to keep them isolated and not pollute the database.
//After the test is done, I can roll back the transaction
// so that no actual data is written to the database
#[tokio::test]
async fn signup_returns_a_200_for_valid_form_data() {
    let config: Config = Config::new();

    let db_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to db");

    let client = hyper::Client::new();

    let body = hyper::Body::from("username=monkey.d.luffy");

    let request = hyper::Request::builder()
        .method(hyper::Method::POST)
        .uri("http://127.0.0.1:8080/users")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .expect("Failed to create request");

    let response = client
        .request(request)
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), hyper::StatusCode::OK);

    let saved = sqlx::query!(
        "select username, referral_code from users where username = 'monkey.d.luffy' ",
        )
        .fetch_one(&db_connection)
        .await
        .expect("Failed to fetch saved data");

    assert_eq!(saved.username, "monkey.d.luffy");
    assert!(!saved.referral_code.is_empty());

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