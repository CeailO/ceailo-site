use std::sync::Arc;

use axum::{
    async_trait,
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod error;

/* trait definition for user repo */
#[async_trait]
trait UserRepo {
    /* look a user by id */
    async fn find(&self, user_id: Uuid) -> Result<User, error::UserRepoError>;

    /* Create a new user */
    async fn create(&self, user_id: Uuid) -> Result<User, error::UserRepoError>;
}

/* Example of `UserRepo` implementation */
struct ExampleUserRepo;

#[async_trait]
impl UserRepo for ExampleUserRepo {
    /* look a user by id */
    async fn find(&self, user_id: Uuid) -> Result<User, error::UserRepoError> {
        todo!()
    }

    /* Create a new user */
    async fn create(&self, user_id: Uuid) -> Result<User, error::UserRepoError> {
        todo!()
    }
}

type DynamicUserRepo = Arc<dyn UserRepo + Send + Sync>;

#[derive(Serialize)]
pub struct User {
    id: Uuid,
    username: String,
}

/* The input to `create_user` handler */
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

impl User {
    pub fn _new(id: Uuid, username: String) -> Self {
        Self { id, username }
    }

    /* handler for `POST` /user */
    pub async fn create_user(
        State(user_repo): State<DynamicUserRepo>,
        Json(payload): Json<CreateUser>,
    ) -> Result<Json<User>, error::AppError> {
        let user = user_repo.create(payload).await?;
        Ok(user.into())
    }

    pub async fn show_user(
        Path(user_id): Path<Uuid>,
        State(user_repo): State<DynamicUserRepo>,
    ) -> Result<Json<User>, error::AppError> {
        let user = user_repo.find(user_id).await?;
        Ok(user.into())
    }
}
