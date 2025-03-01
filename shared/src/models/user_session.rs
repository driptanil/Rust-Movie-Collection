use apistos::ApiComponent;
use sea_orm::{ prelude::{ DateTimeWithTimeZone, Uuid }, Set };
use utoipa::ToSchema;
use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };

use crate::entities::{ user, user_session };
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
pub struct UserSession {
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

impl UserSession {
    pub fn from_model(model: user_session::Model) -> Self {
        Self {
            id: model.id,
            access_token: model.access_token,
            refresh_token: model.refresh_token,
            created_at: model.created_at,
            expires_at: model.expires_at,
            refresh_expires_at: model.refresh_expires_at,
            user_id: model.user_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema, ApiComponent, ToSchema)]
pub struct CreateUserSessionRequest {
    pub access_token: String,
    pub refresh_token: String,
    #[schema(value_type = String, format = "datetime-utc")] pub expires_at: DateTimeWithTimeZone,
    #[schema(
        value_type = String,
        format = "datetime-utc"
    )] pub refresh_expires_at: DateTimeWithTimeZone,
    #[schema(value_type = String, format = "uuid")]
    pub user_id: Uuid,
}

impl CreateUserSessionRequest {
    pub fn to_active_model(&self) -> user_session::ActiveModel {
        user_session::ActiveModel {
            access_token: Set(self.access_token.clone()),
            refresh_token: Set(self.refresh_token.clone()),
            expires_at: Set(self.expires_at.clone()),
            refresh_expires_at: Set(self.refresh_expires_at.clone()),
            user_id: Set(self.user_id),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema, ApiComponent, ToSchema)]
pub struct RefreshUserSessionRequest {
    pub access_token: String,
    pub refresh_token: String,
}
