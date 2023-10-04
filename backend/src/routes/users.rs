use rand::{Rng, thread_rng};
use serde::{Serialize, Deserialize};
use axum::{Router, routing::{get, post}};
use sqlx::{FromRow, types::chrono};

use crate::{root, error::ApiError};
use super::{health_check, signup_username, signup_refcode, view_users};

pub fn router() -> Router {
Router::new()
    .route("/", get(root))
    .route("/health_check", get(health_check))
    .route("/users/view", get(view_users))
    .route("/users/signup-username", post(signup_username))
    .route("/users/signup-refcode", post(signup_refcode))
}

pub fn generate_referral_code(username:String) -> Result<String, ApiError> {
    if username.len() < 3 {
        return Err(ApiError::InvalidUserName);
    }
    let random_num = thread_rng().gen_range(10000..100000);
    let prefix = &username[..3].to_uppercase();
    Ok(format!("{}{}", prefix, random_num))
}

pub fn generate_username(referrer_username: &str) -> Result<String, ApiError> {
    let random_num = thread_rng().gen_range(10000..100000);
    Ok(format!("{}{}", referrer_username,random_num))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsernameInput {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefcodeInput {
    pub referral_code: String,
}

//need to derive FromRow to make data fetchable from database
#[derive(Serialize, FromRow, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub id: i32,
    pub referral_code: String,
    pub referred_by: Option<String>,
    pub invited_users_count: Option<i32>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBody<T> {
    pub user: T,
}