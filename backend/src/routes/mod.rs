use std::sync::Arc;

use axum::{
    routing::{get, patch, post},
    Router,
};

use handler::{handler, handler_404};
use root::_root;

use user::User;

mod handler;
mod root;
mod user;

pub fn create_routes() -> Router {

    let user_repo = Arc::new(ExampleUserRepo) as DynamicUserRepo;

    /* `GET` goes to `root` */
    Router::new()
        .route("/root", patch(_root))
        /* POST goes to `create_user */
        .route("/user", post(User::create_user))
        /* */
        .route("/", get(handler)).fallback(handler_404)
        .with_state(user_repo);
}
