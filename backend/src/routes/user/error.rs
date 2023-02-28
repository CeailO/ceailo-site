use axum::response::IntoResponse;

#[derive(Debug)]
pub enum UserRepoError {
    #[allow(dead_code)]
    NOT_FOUND,
    #[allow(dead_code)]
    INVALID_USERNAME,
}

/* top level error type */
pub enum AppError {
    UserRepo(UserRepoError),
}

/* to enable `?` usage to automatically convert a `userRepoError` */
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
