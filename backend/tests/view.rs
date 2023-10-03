use hyper::Uri;

use backend::config::Config;
use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn view_returns_stored_data(){
    let config: Config = Config::new();

    let db_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to db");

    let client = hyper::Client::new();


    let response = client
        .get(Uri::from_static("http://127.0.0.1:8080/users/view"))
        .await
        .expect("failed to send request");

    assert!(response.status().is_success());  

    let db_results = sqlx::query!(
        "select username from users where username='john_doe'"
    )
    .fetch_one(&db_connection)
    .await
    .expect("Failed to read from db");

    assert_eq!(db_results.username, "john_doe");
}