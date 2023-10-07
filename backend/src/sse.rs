use axum::{response::IntoResponse, Extension};
use sqlx::postgres::PgListener;
use tokio::sync::broadcast::{self, Sender, Receiver, };
use tokio_stream::wrappers::BroadcastStream;
use hyper::{Response, StatusCode};
use tracing::error;
use tokio_stream::StreamExt;
use crate::{startup::ApiContext, error::ApiError, routes::users::User};


///Route handlers for Server Sent Events
pub async fn sse_handler(ctx: Extension<ApiContext>) -> Result<impl IntoResponse, ApiError> {
    let (tx, rx): (Sender<String>, Receiver<String>) = broadcast::channel(100);

    let mut listener = PgListener::connect_with(&ctx.db).await.map_err(|err| {
        error!("Error connecting with PgListener: {}", err);
        ApiError::InternalServerError
    })?;

    listener.listen("user_changes").await.map_err(|err| {
        error!("Error listening to user_changes channel: {}", err);
        ApiError::InternalServerError
    })?;

    tokio::spawn(async move {
        let mut notification_stream = listener.into_stream();

        while let Some(notification) = notification_stream.next().await {
            if notification.is_ok() {
                // Fetch updated data from the db
                let users_result = sqlx::query_as!(
                    User,
                    r#"
                        select * from users order by invited_users_count
                    "#
                )
                .fetch_all(&ctx.db)
                .await;

                match users_result {
                    Ok(users) => {
                        let message = format!("data: {}\n\n", serde_json::to_string(&users).unwrap());
                        let _ = tx.send(message);
                    }
                    Err(err) => {
                        error!("Error fetching user data from db: {}", err);
                    }
                }
            }
        }
    });

    let stream = BroadcastStream::new(rx);

    // Construct the SSE response with appropriate headers
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/event-stream")
        .header("cache-control", "no-cache")
        .header("connection", "keep-alive")
        .body(hyper::Body::wrap_stream(stream))
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(response)
}