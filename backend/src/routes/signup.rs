use axum::{Extension, Json};
use axum::extract::Form;
use axum_macros::debug_handler;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use tracing::{error, debug};
use jsonwebtoken::{ encode, EncodingKey, Header };
use crate::routes::users::{ UsernameInput, User, generate_referral_code, generate_username };
use crate::startup::ApiContext;
use crate::error::ApiError;

use super::users::RefcodeInput;

#[debug_handler]
pub async fn signup_username(
    ctx: Extension<ApiContext>, 
    Form(input): Form<UsernameInput>
) -> Result<Json<User>, ApiError> {
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

            if let sqlx::Error::Database(db_err) = &err {
                if db_err.code().as_deref() == Some("23505") {
                   return ApiError::UserAlreadyExists;
                }
            }

            error!("error trying to insert into db: {}", err);
            ApiError::InternalServerError
        })?;

        Ok(Json(user))
    }
}

#[debug_handler]
pub async fn signup_refcode(
    ctx: Extension<ApiContext>,
    Form(input): Form<RefcodeInput>
) -> Result<Json<User>, ApiError> {
    
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

    //create a database transaction in order to be able
    //to insert and update the db atomically
    let mut tx = ctx.db.begin().await.map_err(| err | {
        error!("error starting database transaction: {err}");
        ApiError::InternalServerError
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
    .fetch_one(&mut *tx)
    .await
    .map_err(| err | {
        error!("error creating new user from username: {err}");
        ApiError::InternalServerError
    })?;

    //Increment the referrer's invited_users_count
    sqlx::query!(
        r#"
            update users
            set invited_users_count = invited_users_count + 1
            where referral_code = $1
        "#,
        input.referral_code
    )
    .execute(&mut *tx)
    .await
    .map_err(| err | {
        error!("error updating referrer's invited_users_count: {err}");
        ApiError::InternalServerError
    })?;

    //commit the transaction
    tx.commit().await.map_err(| err | {
        error!("error committing the transaction: {err}");
        ApiError::InternalServerError
    })?;

    Ok(Json( new_user ))

}

///Claims for JWTs
#[derive(Serialize, Deserialize)]
struct Claims {
    username: String,
    exp: i64,
}

pub async fn login(
    ctx: Extension<ApiContext>,
    Form(input): Form<UsernameInput>
) -> Result<Json<Value>, ApiError> {

    let user = sqlx::query_as!(
        User,
        r#"
            select * from users where username = $1
        "#,
        input.username
    )
    .fetch_one(&ctx.db)
    .await
    .map_err(| err | {
        error!("Error trying to login: {}", err);
        ApiError::UserDoesNotExist
    })?;

    //set the token expiration time
    let expiration_duration = chrono::Duration::minutes(10);
    let expiration = (Utc::now() + expiration_duration).timestamp();

    //Create the claims for the JWT
    let claims = Claims{
        username: input.username.clone(),
        exp: expiration,
    };

    //Generate the JWT
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("som_key".as_ref()))
        .map_err(|_| ApiError::TokenCreationFailed)?;

    let response_data = json!({
        "token": token,
        "user": user,
    });

    Ok(Json(response_data))
}