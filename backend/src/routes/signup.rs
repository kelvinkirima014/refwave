use axum::Json;
use axum::extract::Extension;
use axum_macros::debug_handler;
//use serde_json::json;
use crate::routes::users::{ UserInput, User, generate_referral_code };
use crate::startup::ApiContext;
use crate::error::ApiError;

#[debug_handler]
pub async fn signup(
    ctx: Extension<ApiContext>, 
    Json(input): Json<UserInput>
) -> Result<Json<User>, ApiError> {
    let new_referral_code = generate_referral_code(input.username.clone().unwrap());

    if let Some(username) = input.username {
        if username.is_empty() {
            return Err(ApiError::MissingCredential);
        } else {
            sqlx::query!(
                r#"
                    insert into users (username, referral_code)
                    values ($1, $2)
                "#,
                username,
                new_referral_code
            )
            .execute(&ctx.db)
            .await
            .map_err(|err| {
                dbg!(err);
                ApiError::InternalServerError
            })?;
            Ok(Json(User { 
                username, 
                referral_code: Some(new_referral_code) 
            }))
        }
    } else {
        Err(ApiError::InternalServerError)
    }





    // if let Some(username) = &input.username {
    //     if username.is_empty() {
    //         return Ok(response::Response::builder()
    //             .status(StatusCode::BAD_REQUEST)
    //             .body(hyper::Body::from("Username can not be empty"))
    //             .unwrap());
    //     } else {
    //         sqlx::query!(
    //             r#"
    //             INSERT INTO users (username, referral_code)
    //             VALUES ($1, $2)
    //             "#,
    //             username,
    //             new_referral_code
    //         )
    //         .execute(&ctx.db)
    //         .await
    //         .expect("Failed to insert into database");

    //         return Ok(response::Response::builder()
    //             .status(StatusCode::OK)
    //             .body(Body::from("Signup succesful"))
    //             .unwrap());
    //     } 
            
       
    // } else {
    //     return Ok(response::Response::builder()
    //     .status(StatusCode::BAD_REQUEST)
    //     .body(Body::from("Missing username"))
    //     .unwrap());

    // }

}