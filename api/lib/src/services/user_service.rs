use actix_web::{ error::ErrorUnauthorized, Error };
use chrono::{ Duration, Utc };
use jsonwebtoken::{ decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation };
use shared::models::{
    user::{ CreateUserRequest, LoginRequest, User, UserClaims },
    user_password::{ CreateUserPassword, UserPassword },
    user_session::{ CreateUserSessionRequest, RefreshUserSessionRequest, UserSession },
};
use std::sync::Arc;
use crate::{
    repositories::{
        user_repository::UserRepository,
        user_session_repository::UserSessionRepository,
        user_password_repository::UserPasswordRepository,
    },
    utils::error::{ ApiError, ApiResult },
};

#[async_trait::async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn get_token(&self, req: LoginRequest) -> ApiResult<UserSession>;
    async fn refresh_token(&self, req: RefreshUserSessionRequest) -> ApiResult<UserSession>;
    async fn register(&self, req: CreateUserRequest) -> ApiResult<UserSession>;
}

pub struct UserServiceImpl {
    repo: Arc<dyn UserRepository>,
    session_repo: Arc<dyn UserSessionRepository>,
    password_repo: Arc<dyn UserPasswordRepository>,
}

impl UserServiceImpl {
    pub fn new(
        repo: Arc<dyn UserRepository>,
        session_repo: Arc<dyn UserSessionRepository>,
        password_repo: Arc<dyn UserPasswordRepository>
    ) -> Self {
        Self { repo, session_repo, password_repo }
    }

    fn generate_jwt_token(user: User) -> (String, i64) {
        let secret = std::env::var("AUTH_SECRET").unwrap_or_else(|_| "default_secret".to_string());

        let expires: i64 = (Utc::now() + Duration::minutes(120)).timestamp();

        let claims = user.to_claims(expires);

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes())
        );

        match token {
            Ok(t) => (t, expires),
            Err(e) => panic!("Error generating token: {}", e),
        }
    }

    fn generate_refresh_token(user: User) -> (String, i64) {
        let secret = std::env
            ::var("REFRESH_SECRET")
            .unwrap_or_else(|_| "refresh_secret".to_string());

        let expires: i64 = (Utc::now() + Duration::days(30)).timestamp();

        let claims = user.to_claims(expires);

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes())
        );

        match token {
            Ok(t) => (t, expires),
            Err(e) => panic!("Error generating refresh token: {}", e),
        }
    }

    fn validate_refresh_token(&self, refresh_token: &str) -> Result<User, Error> {
        let secret = std::env
            ::var("REFRESH_SECRET")
            .unwrap_or_else(|_| "refresh_secret".to_string());

        tracing::info!("Validating refresh token");

        match
            decode::<UserClaims>(
                refresh_token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::new(Algorithm::HS256)
            )
        {
            Ok(token_data) => {
                let user = User {
                    id: token_data.claims.id,
                    name: token_data.claims.name,
                    email: token_data.claims.email,
                    email_verified: None, // Set to None or provide a default
                    image: None, // Set to None or provide a default
                    created_at: Utc::now().into(), // Provide a default or handle accordingly
                    updated_at: Utc::now().into(), // Provide a default or handle accordingly
                };
                Ok(user)
            }
            Err(e) => {
                tracing::error!("Refresh token validation failed: {}", e);
                Err(ErrorUnauthorized("Invalid or expired refresh token"))
            }
        }
    }

    async fn generate_session(&self, user: User) -> ApiResult<UserSession> {
        let (jwt_token, expires) = Self::generate_jwt_token(user.clone());
        let (refresh_token, refresh_expires) = Self::generate_refresh_token(user.clone());

        let now_utc = Utc::now();

        let expires_at_utc = now_utc + Duration::seconds(expires);
        let refresh_expires_at_utc = now_utc + Duration::seconds(refresh_expires);

        let create_user_session = CreateUserSessionRequest {
            access_token: jwt_token.clone(),
            refresh_token: refresh_token.clone(),
            expires_at: expires_at_utc.into(),
            refresh_expires_at: refresh_expires_at_utc.into(),
            user_id: user.id,
        };

        let inserted_user_session = self.session_repo
            .create_user_session(create_user_session.to_active_model()).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        Ok(UserSession::from_model(inserted_user_session))
    }
}

#[async_trait::async_trait]
impl UserService for UserServiceImpl {
    async fn get_token(&self, req: LoginRequest) -> ApiResult<UserSession> {
        let user = self.repo
            .get_user_by_email(req.email).await
            .map_err(|e| { ApiError::InternalServer(format!("Database error: {:?}", e)) })?;

        if let None = user {
            return Err(ApiError::BadRequest(format!("User not found")));
        }

        let user = User::from_model(user.unwrap());

        let user_password = self.password_repo
            .get_by_user_id(user.id).await
            .map_err(|e| { ApiError::InternalServer(format!("Database error: {:?}", e)) })?;

        if let None = user_password {
            return Err(ApiError::NotFound(format!("User not found")));
        }

        let user_password = UserPassword::from_model(user_password.unwrap());

        let is_valid = bcrypt::verify(req.password, &user_password.password).unwrap();

        if !is_valid {
            return Err(ApiError::BadRequest(format!("Incorrect Password")));
        }

        self.generate_session(user.clone()).await
    }

    async fn refresh_token(&self, req: RefreshUserSessionRequest) -> ApiResult<UserSession> {
        let user = self
            .validate_refresh_token(&req.refresh_token)
            .map_err(|e| { ApiError::Unauthorized(format!("Invalid or expired refresh token")) })?;

        self.session_repo
            .get_session(req.access_token.clone(), req.refresh_token.clone()).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        self.generate_session(user.clone()).await
    }

    async fn register(&self, req: CreateUserRequest) -> ApiResult<UserSession> {
        let new_user = req.to_active_model();

        let find_user = self.repo
            .get_user_by_email(req.email).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        if find_user.is_some() {
            return Err(ApiError::BadRequest(format!("Email address already exists")));
        }

        let inserted_user = self.repo
            .create(new_user).await
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

        self.password_repo
            .create(user_password.to_active_model()).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        self.generate_session(user.clone()).await
    }
}
