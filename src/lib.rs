use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};

use crate::config::Config;
use crate::user::User;

mod config;
mod user;

pub async fn run() {
    /* initialize tracing */
    tracing_subscriber::fmt::init();

    // build app with routes
    let app = Router::new()
        /* `GET` goes to `root` */
        .route("/", get(root))
        /* `POST` goes to `create_user` */
        .route("/user", post(User::create_user));
    /* */
    // .route("/index", get_service(ServeFile::new("static/index.html")));

    /* run app with hyper
     * `axum::Server` is a re-export of `hyper::Server`
     */
    let add = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", add);
    axum::Server::bind(&add)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "root"
}
