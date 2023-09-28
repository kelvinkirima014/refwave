use rand::{Rng, thread_rng};
use serde::{Serialize, Deserialize};
use axum::{Router, routing::{get, post}};

use crate::root;
use super::{health_check, signup};

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health_check", get(health_check))
        .route("/users", post(signup))
}

pub(crate) fn generate_referral_code(username:String) -> String {
    let random_num = thread_rng().gen_range(10000..100000);
    let prefix = &username[..3].to_uppercase();
    format!("{}{}", prefix, random_num)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UserInput {
    pub username: Option<String>,
    pub referral_code: Option<String>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub referral_code: Option<String>,
}
