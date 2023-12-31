use axum:: {
    http::StatusCode,
    response::{ IntoResponse, Response },
    Json
};
use serde_json::json;


pub enum ApiError {
    InternalServerError,
    MissingCredential,
    InvalidUserName,
    UserAlreadyExists,
    UserDoesNotExist,
    TokenCreationFailed,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occured",
            ),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "missing credential"),
            Self::InvalidUserName => (StatusCode::BAD_REQUEST, "invalid username"),
            Self::UserAlreadyExists => (StatusCode::BAD_REQUEST, "user already exists"),
            Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "user does not exist"),
            Self::TokenCreationFailed =>(StatusCode::UNAUTHORIZED, "input valid login creadentials"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}