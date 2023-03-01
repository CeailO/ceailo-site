use std::sync::Arc;

use axum::async_trait;
use uuid::Uuid;

use super::{
    error::UserRepoError,
    user::{CreateUser, User},
};

/* trait definition for user repo */
#[async_trait]
pub trait UserRepo {
    /* look a user by id */
    async fn find(&self, user_id: Uuid) -> Result<User, UserRepoError>;

    /* Create a new user */
    async fn create(&self, _param: CreateUser) -> Result<User, UserRepoError>;
}

/* Example of `UserRepo` implementation */
pub struct ExampleUserRepo;

#[async_trait]
impl UserRepo for ExampleUserRepo {
    /* look a user by id */
    async fn find(&self, _user_id: Uuid) -> Result<User, UserRepoError> {
        todo!()
    }

    /* Create a new user */
    async fn create(&self, _param: CreateUser) -> Result<User, UserRepoError> {
        todo!()
    }
}

pub type _DynamicUserRepo = Arc<dyn UserRepo + Send + Sync>;
