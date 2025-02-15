use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use shared::models::user::User;

pub fn generate_jwt_token(user: User) -> (String, i64) {

    let expires:i64 = (Utc::now() + Duration::minutes(1)).timestamp();

    let token = encode(&Header::default(), &user.to_claims(expires
    ), &EncodingKey::from_secret("AUTH_SECRET".as_bytes()));


    match token {
        Ok(t) => (t, expires),
        Err(e) => panic!("Error generating token: {}", e)
    }
}