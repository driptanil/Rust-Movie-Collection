use actix_web::{ dev::ServiceRequest, error::ErrorUnauthorized, Error, HttpMessage, HttpRequest };
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{ Duration, Utc };
use jsonwebtoken::{ decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation };
use shared::models::user::{ User, UserClaims };

pub fn generate_jwt_token(user: User) -> (String, i64) {
    let secret = std::env::var("AUTH_SECRET").unwrap_or_else(|_| "default_secret".to_string());

    let expires: i64 = (Utc::now() + Duration::minutes(1)).timestamp();

    let claims = user.to_claims(expires);

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()));

    match token {
        Ok(t) => (t, expires),
        Err(e) => panic!("Error generating token: {}", e),
    }
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();

    tracing::info!("Validating token");

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
