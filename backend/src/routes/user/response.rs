use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use super::error::{AppError, UserRepoError};

/* enable `?` to be used and automatically convert a `UserRepoError` */
impl From<UserRepoError> for AppError {
    fn from(inner: UserRepoError) -> Self {
        AppError::UserRepo(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::UserRepo(UserRepoError::NotFound) => {
                (StatusCode::NOT_FOUND, "user not found")
            }
            AppError::UserRepo(UserRepoError::InvalidUsername) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "invalid username")
            }
        };
        let body = Json(json!({ "error": error_message }));

        (status, body).into_response()
    }
}
