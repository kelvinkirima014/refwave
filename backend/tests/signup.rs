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

    let body = hyper::Body::from("username=luffy");

    let request = hyper::Request::builder()
        .method(hyper::Method::POST)
        .uri("http://127.0.0.1:8080/users/signup-username")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .expect("Failed to create request");

    let response = client
        .request(request)
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), hyper::StatusCode::OK);

    let saved = sqlx::query!(
        "select username, referral_code from users where username = 'luffy' ",
        )
        .fetch_one(&db_connection)
        .await
        .expect("Failed to fetch saved data");

    assert_eq!(saved.username, "luffy");
    assert!(!saved.referral_code.is_empty());

}

#[tokio::test]
async fn refcode_signup_returns_a_200_for_valid_form_data() {
    let config: Config = Config::new();

    let db_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to db");

    let client = hyper::Client::new();

    let body = hyper::Body::from("referral_code=JOHN12345");

    let request = hyper::Request::builder()
        .method(hyper::Method::POST)
        .uri("http://127.0.0.1:8080/users/signup-refcode")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .expect("Failed to create request");


    let response = client
        .request(request)
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), hyper::StatusCode::OK);

    let saved = sqlx::query!(
        "select username, referral_code from users where referral_code = 'JOHN12345' ",
        )
        .fetch_one(&db_connection)
        .await
        .expect("Failed to fetch saved data");

    assert_eq!(saved.username, "john_doe");
    assert!(!saved.referral_code.is_empty());


}
