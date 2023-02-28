use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Test</h1>")
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here")
}
