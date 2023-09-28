use axum:: {
    http::StatusCode,
    response::{ IntoResponse, Response },
    Json
};
use serde_json::json;


pub enum ApiError {
    InternalServerError,
    MissingCredential,
    UserAlreadyExists,
    UserDoesNotExist
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occured",
            ),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "missing credential"),
            Self::UserAlreadyExists => (StatusCode::BAD_REQUEST, "user already exists"),
            Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "User does not exist")
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}