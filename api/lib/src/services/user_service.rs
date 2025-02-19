use chrono::{ Duration, Utc };
use shared::{
    models::{
        user::{ CreateUserRequest, LoginRequest, User, UserToken },
        user_password::{ CreateUserPassword, UserPassword },
    },
};
use std::sync::Arc;
use crate::{
    repositories::user_repository::UserRepository,
    utils::{ auth::generate_jwt_token, error::{ ApiError, ApiResult } },
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

        let user_password = self.repo
            .get_user_password(user.id).await
            .map_err(|e| { ApiError::InternalServer(format!("Database error: {:?}", e)) })?;

        if let None = user_password {
            return Err(ApiError::NotFound(format!("User not found")));
        }

        let user_password = UserPassword::from_model(user_password.unwrap());

        let is_valid = bcrypt::verify(req.password, &user_password.password).unwrap();

        if !is_valid {
            return Err(ApiError::BadRequest(format!("Incorrect Password")));
        }

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

        let find_user = self.repo
            .get_user_by_email(req.email).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        if find_user.is_some() {
            return Err(ApiError::BadRequest(format!("Email address already exists")));
        }

        let inserted_user = self.repo
            .create_user(new_user).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        let user = User::from_model(inserted_user);

        let hashed_password = bcrypt
            ::hash(req.password, 10)
            .map_err(|e| { ApiError::InternalServer("Cryptography error".to_owned()) })?;

        let user_password = CreateUserPassword {
            user_id: user.id,
            password: hashed_password,
            password_salt: "".to_owned(),
        };

        let inserted_user_password = self.repo
            .create_user_password(user_password.to_active_model()).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        Ok(user)
    }
}
