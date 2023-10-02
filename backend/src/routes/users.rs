use rand::{Rng, thread_rng};
use serde::{Serialize, Deserialize};
use axum::{Router, routing::{get, post}};
use sqlx::FromRow;

use crate::{root, error::ApiError};
use super::{health_check, signup_username};

pub fn router() -> Router {
Router::new()
    .route("/", get(root))
    .route("/health_check", get(health_check))
    .route("/users/signup", post(signup_username))
}

pub fn generate_referral_code(username:String) -> Result<String, ApiError> {
if username.len() < 3 {
    return Err(ApiError::InvalidUserName);
}
let random_num = thread_rng().gen_range(10000..100000);
let prefix = &username[..3].to_uppercase();
Ok(format!("{}{}", prefix, random_num))
}


//will probably need to derive sqlx::FromRow to make data
//fetchable from the database

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct UsernameInput {
    pub username: String,
}
pub struct RefcodeInput {
    pub refferal_code: String,
}


#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub id: i32,
    pub referral_code: String,
    pub invited_users_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBody<T> {
    pub user: T,
}