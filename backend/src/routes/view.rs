use axum::{Extension, Json};
use serde_json::{Value, json};
use tracing::error;

use crate::{startup::ApiContext, error::ApiError};
use crate::routes::users::User;

pub async fn view_users(
    ctx: Extension<ApiContext>,
) -> Result<Json<Value>, ApiError> {
    let users = sqlx::query_as!(
        User,
        r#"
            select * from users order by id
        "#
    )
    .fetch_all(&ctx.db)
    .await
    .map_err(| err | {
        error!("Error trying to get fetch users: {err}");
        ApiError::InternalServerError
    })?;

    Ok(Json(json!(users)))
}