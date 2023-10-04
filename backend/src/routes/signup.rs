use axum::{Extension, Json};
use axum::extract::Form;
use axum_macros::debug_handler;
use tracing::{error, debug};
//use serde_json::json;
use crate::routes::users::{ UsernameInput, User, generate_referral_code, generate_username };
use crate::startup::ApiContext;
use crate::error::ApiError;

use super::users::{UserBody, RefcodeInput};

#[debug_handler]
pub async fn signup_username(
    ctx: Extension<ApiContext>, 
    Form(input): Form<UsernameInput>
) -> Result<Json<UserBody<User>>, ApiError> {
    debug!("Received signup request for username: {}", input.username);


    let referral_code = generate_referral_code(input.username.clone())?;

    if input.username.is_empty(){
        return Err(ApiError::MissingCredential);
    } else {
        debug!("Inserting user: {} with referral code: {}", input.username, referral_code);
        let user = sqlx::query_as!(
            User,
            r#"
                insert into users(username, referral_code)
                values($1, $2)
                returning *
            "#,
            input.username,
            referral_code
        )
        .fetch_one(&ctx.0.db)
        .await
        .map_err(| err | {
            error!("error trying to insert into db: {}", err);
            ApiError::InternalServerError
        })?;
        debug!("Successfully inserted user: {:?}", user);

        Ok(Json(UserBody {
           user,
        }))
    }
}

#[debug_handler]
pub async fn signup_refcode(
    ctx: Extension<ApiContext>,
    Form(input): Form<RefcodeInput>
) -> Result<Json<UserBody<User>>, ApiError> {
    debug!("signing up user with referral code!");

    if input.referral_code.is_empty() {
        return Err(ApiError::MissingCredential);
    }

    let referrer = sqlx::query_as!(
        User,
        r#"
            select * from users where referral_code = $1   
        "#,
        input.referral_code
    )
    .fetch_one(&ctx.db)
    .await
    .map_err(| err | {
        error!("error fetching user by referral code from DB: {err}");
        ApiError::UserDoesNotExist
    })?;

    let new_username = generate_username(&referrer.username)?;
    let new_referral_code = generate_referral_code(new_username.clone())?;
    let new_user = sqlx::query_as!(
        User,
        r#"
            insert into users (username, referral_code, referred_by)
            values ($1, $2, $3)
            returning *
        "#,
        new_username,
        new_referral_code,
        referrer.referral_code
    )
    .fetch_one(&ctx.db)
    .await
    .map_err(| err | {
        error!("error creating new user from username: {err}");
        ApiError::InternalServerError
    })?;

    Ok(Json(UserBody { user: new_user }))

}