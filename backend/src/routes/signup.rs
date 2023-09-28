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
    
    let referral_code_result = generate_referral_code(input.username.clone().unwrap());

    // Handle the error from generate_referral_code
    let new_referral_code = match referral_code_result {
        Ok(code) => code,
        Err(e) => return Err(e),
    };


    if let Some(username) = input.username {
        if username.is_empty() && input.referral_code.is_none() {
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
}