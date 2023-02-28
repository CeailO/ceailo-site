use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

/* The input to `create_user` handler */
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

impl User {
    pub fn _new(id: u64, username: String) -> Self {
        Self { id, username }
    }

    pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
        /*
         * this argument tells axum to parse the request body as json
         */
        let user = Self {
            id: 1337,
            username: payload.username,
        };
        (StatusCode::CREATED, Json(user))
    }
}
