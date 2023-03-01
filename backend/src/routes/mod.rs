use std::{sync::Arc, future::Future, process::Output};

use axum::{
    routing::{get, patch, post},
    Router, handler::Handler,
};

use handler::{handler, handler_404};
use root::_root;

use self::user::{
    user::User,
    user_repo::{DynamicUserRepo, ExampleUserRepo},
};

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
        // .route("/user/:id", get((User::show_user))) // Until uuid is implemented officially
        /* */
        .route("/", get(handler))
        .fallback(handler_404)
        .with_state(user_repo)
}
