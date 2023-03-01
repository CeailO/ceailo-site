use std::sync::Arc;

use axum::{
    routing::{get, patch, post},
    Router,
};

use handler::{_handler, _handler_404};
use root::_root;

use self::user::{
    user::User,
    user_repo::{_DynamicUserRepo, ExampleUserRepo},
};

mod handler;
mod root;
mod user;

pub fn _create_routes() -> Router {
    let user_repo = Arc::new(ExampleUserRepo) as _DynamicUserRepo;

    /* `GET` goes to `root` */
    Router::new()
        .route("/root", patch(_root))
        /* POST goes to `create_user */
        .route("/user", post(User::_create_user))
        /* */
        // .route("/user/:id", get((User::_show_user))) // Until uuid is implemented officially
        /* */
        .route("/", get(_handler))
        .fallback(_handler_404)
        .with_state(user_repo)
}
