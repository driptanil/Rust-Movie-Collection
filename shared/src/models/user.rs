use apistos::ApiComponent;
use sea_orm::{prelude::{ DateTimeWithTimeZone, Uuid }, Set};
use utoipa::ToSchema;
use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };

use crate::entities::user;
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    JsonSchema,
    ApiComponent,
    ToSchema
)]
pub struct User {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub email_verified: Option<DateTimeWithTimeZone>,
    pub image: Option<String>,
    #[schema(value_type = String, format = "datetime-utc")]
    pub created_at: DateTimeWithTimeZone,
    #[schema(value_type = String, format = "datetime-utc")]
    pub updated_at: DateTimeWithTimeZone,
}

impl User {
    pub fn from_model(model: user::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            email: model.email,
            email_verified: model.email_verified,
            image: model.image,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }

    pub fn to_claims(&self, expires: i64) -> UserClaims {
        UserClaims {
            id: self.id,
            email: self.email.clone(),
            expires,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub id: Uuid,
    pub email: String,
    expires: i64,
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Default,
    JsonSchema,
    ApiComponent,
    ToSchema
)]
pub struct UserToken {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    #[schema(value_type = String, format = "datetime-utc")]
    pub created_at: DateTimeWithTimeZone,
    #[schema(value_type = String, format = "datetime-utc")]
    pub expires_at: DateTimeWithTimeZone,
    #[schema(value_type = String, format = "datetime-utc")]
    pub refresh_expires_at: DateTimeWithTimeZone,

    #[schema(value_type = String, format = "uuid")]
    pub user_id: Uuid,
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Default,
    JsonSchema,
    ApiComponent,
    ToSchema
)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Default,
    JsonSchema,
    ApiComponent,
    ToSchema
)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl CreateUserRequest {
    pub fn to_active_model(&self) -> user::ActiveModel {
        user::ActiveModel {
            email: Set(self.email.clone()),
            name: Set(self.name.clone()),
            ..Default::default()
        }
    }
}