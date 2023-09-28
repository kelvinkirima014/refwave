use std::convert::Infallible;

use axum::{ response::{self, IntoResponse}, extract };
use hyper::{StatusCode, Body};
use crate::routes::users::{ UserInput,generate_referral_code };
use crate::startup::ApiContext;

pub async fn signup(
    ctx: extract::Extension<ApiContext>, 
    input: extract::Form<UserInput>
) -> Result<impl IntoResponse, Infallible> {

    //let db_pool = PgPoolOptions::new();

    let new_referral_code = generate_referral_code(input.username.clone().unwrap());

    if let Some(username) = &input.username {
        if username.is_empty() {
            return Ok(response::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(hyper::Body::from("Username can not be empty"))
                .unwrap());
        } else {
            sqlx::query!(
                r#"
                INSERT INTO users (username, referral_code)
                VALUES ($1, $2)
                "#,
                username,
                new_referral_code
            )
            .execute(&ctx.db)
            .await
            .expect("Failed to insert into database");

            return Ok(response::Response::builder()
                .status(StatusCode::OK)
                .body(Body::from("Signup succesful"))
                .unwrap());
        } 
            
       
    } else {
        return Ok(response::Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("Missing username"))
        .unwrap());

    }

}