use axum::{
    body::Body,
    extract::{Path, State},
    handler::Handler,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{fmt::writer::EitherWriter, Layer};
use uuid::Uuid;

use super::{
    custom_uuid,
    error::AppError,
    user_repo::{self, DynamicUserRepo},
};

#[derive(Debug, Serialize, Clone)]
pub struct User {
    #[serde(with = "custom_uuid")]
    id: Uuid,
    username: String,
}

/* The input to `create_user` handler */
#[derive(Deserialize, Clone)]
pub struct CreateUser {
    username: String,
}

impl User {
    pub fn _new(id: Uuid, username: String) -> Self {
        Self {
            id: id,
            username: username,
        }
    }

    /* handler for `POST` /user */
    pub async fn create_user(
        State(user_repo): State<DynamicUserRepo>,
        Json(payload): Json<CreateUser>,
    ) -> Result<Json<User>, AppError> {
        let user = user_repo.create(payload).await?;
        Ok(user.into())
    }

    pub async fn show_user(
        Path(user_id): Path<Uuid>,
        State(user_repo): State<DynamicUserRepo>,
    ) -> Result<Json<User>, AppError> {
        let user = user_repo.find(user_id).await?;
        Ok(user.into())
    }
}

// new() is private. There's nowhere to go here

// impl Handler<(), State<DynamicUserRepo>, EitherWriter> for User {
//     type Future;

//     fn call(self, req: axum::http::Request<Body>, state: State<DynamicUserRepo>) -> Self::Future {
//         todo!()
//     }

//     fn layer<L, NewReqBody>(self, layer: L) -> axum::handler::Layered<L, Self, (), State<DynamicUserRepo>, EitherWriter, NewReqBody>
//     where
//         L: Layer<axum::handler::HandlerService<Self, (), State<DynamicUserRepo>, EitherWriter>> + Clone,
//         L::Service: Service<axum::http::Request<NewReqBody>>,
//     {
//         axum::handler::Layered {
//             layer,
//             handler: self,
//             _marker: std::marker::PhantomData,
//         }
//     }

//     fn with_state(self, state: State<DynamicUserRepo>) -> axum::handler::HandlerService<Self, (), State<DynamicUserRepo>, EitherWriter> {
//         axum::handler::HandlerService::new(self, state)
//     }
// }
