use actix_web::{ dev::ServiceRequest, error::ErrorUnauthorized, web, Error, HttpMessage };

use actix_web_httpauth::extractors::{basic::BasicAuth, bearer::BearerAuth};
use chrono::Utc;
use jsonwebtoken::{ decode, Algorithm, DecodingKey, Validation };
use shared::models::user::{ LoginRequest, User, UserClaims };

use crate::services::user_service::{UserService, UserServiceImpl};

pub async fn auth_middleware(
    req: ServiceRequest,
    credentials: BearerAuth
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();

    tracing::info!("Validating token {}", token);

    let secret = std::env::var("AUTH_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    match decode::<UserClaims>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation) {
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
            req.extensions_mut().insert(user); // Insert the user into the request extensions
            Ok(req)
        }
        Err(e) => {
            tracing::error!("Token validation failed: {}", e);
            Err((ErrorUnauthorized("Invalid or expired token"), req))
        }
    }
}

pub async fn basic_auth_middleware(
    req: ServiceRequest,
    credentials: BasicAuth
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let email = credentials.user_id().to_string();
    let password = credentials.password().unwrap_or_default().to_string();

    tracing::info!("Validating basic auth credentials for: {}", email);

    type UserServiceType = UserServiceImpl;

   
    let user_service = match req.app_data::<web::Data<UserServiceType>>() {
        Some(service) => service,
        None => {
            tracing::error!("User service not available");
            return Err((ErrorUnauthorized("User service not configured"), req));
        }
    };

    // Check credentials using the service interface
    match user_service.get_token(LoginRequest{
        email: email.clone(),
        password: password.clone()
    }).await {
        Ok(user) => {
            // If authentication is successful, store the user in the request extensions
            req.extensions_mut().insert(user);
            Ok(req)
        },
        Err(_) => {
            tracing::error!("Invalid credentials for: {}", email);
            Err((ErrorUnauthorized("Invalid credentials"), req))
        }
    }
}