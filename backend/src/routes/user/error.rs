use axum::response::IntoResponse;

#[derive(Debug)]
pub enum UserRepoError {
    #[allow(dead_code)]
    NotFound,
    #[allow(dead_code)]
    InvalidUsername,
}

/* top level error type */
pub enum AppError {
    UserRepo(UserRepoError),
}
