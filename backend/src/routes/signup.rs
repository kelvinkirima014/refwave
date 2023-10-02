use axum::Extension;
use axum::extract::Form;
use axum_macros::debug_handler;
use tracing::error;
//use serde_json::json;
use crate::routes::users::{ UsernameInput, User, generate_referral_code };
use crate::startup::ApiContext;
use crate::error::ApiError;

use super::users::UserBody;

#[debug_handler]
pub async fn signup_username(
    ctx: Extension<ApiContext>, 
    Form(input): Form<UserBody<UsernameInput>>
) -> Result<Form<UserBody<User>>, ApiError> {

    let referral_code = generate_referral_code(input.user.username.clone())?;

    if input.user.username.is_empty(){
        return Err(ApiError::MissingCredential);
    } else {
        let insert_user  = sqlx::query_as!(
            User,
            r#"
                insert into users(username, referral_code)
                values($1, $2)
                returning username, id, referral_code, invited_users_count
            "#,
            input.user.username,
            referral_code
        )
        .fetch_one(&ctx.0.db)
        .await
        .map_err(|err| {
            error!("Error trying to insert to the database: {}", err);
            ApiError::InternalServerError
        })?;

        Ok(Form(UserBody { user: insert_user }))        
    }
}