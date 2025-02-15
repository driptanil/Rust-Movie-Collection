
use chrono::{DateTime, Duration, Utc};
use shared::{entities::user, models::user::{CreateUserRequest, LoginRequest, User, UserToken}};
use sea_orm::prelude::{DateTimeWithTimeZone, Uuid};
use std::sync::Arc;
use crate::{
    repositories::user_repository::UserRepository,
    utils::{auth::generate_jwt_token, error::{ ApiError, ApiResult }},
};

#[async_trait::async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn get_token(&self, req: LoginRequest) -> ApiResult<UserToken>;
    async fn register(&self, req: CreateUserRequest) -> ApiResult<User>;
}


pub struct UserServiceImpl {
    repo: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl UserService for UserServiceImpl {
    async fn get_token(&self, req: LoginRequest) -> ApiResult<UserToken> {
        let user = self.repo
            .get_user_by_email(req.email).await
            .map_err(|e| { ApiError::InternalServer(format!("Database error: {:?}", e)) })?;

        if let None = user {
            return Err(ApiError::BadRequest(format!("User not found")));
        }

        let user = User::from_model(user.unwrap()); // Unwrap the user once and reuse it
        
        let (jwt_token, expires) = generate_jwt_token(user.clone());
        let now_utc = Utc::now();

        let expires_at_utc = now_utc + Duration::seconds(expires);
        let refresh_expires_at_utc = now_utc + Duration::days(30);

        let now = now_utc.into();
        let expires_at = expires_at_utc.into();
        let refresh_expires_at = refresh_expires_at_utc.into();

        Ok(UserToken {
            id: user.id,
            access_token: jwt_token,
            refresh_token: "refresh token".to_string(),
            created_at: now,
            expires_at,
            refresh_expires_at,
            user_id: user.id,
        })
            
    }

    async fn register(&self, req: CreateUserRequest) -> ApiResult<User> {
        let new_user = req.to_active_model();

        let inserted_user = self.repo
            .create_user(new_user).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        Ok(User::from_model(inserted_user))
    }
}
