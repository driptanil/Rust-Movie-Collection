use apistos::ApiComponent;
use sea_orm::{ prelude::{ DateTimeWithTimeZone, Uuid }, Set };
use utoipa::ToSchema;
use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };

use crate::entities::{ user, user_password };
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
pub struct UserPassword {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub password: String,
    pub user_id: Uuid,
    #[schema(value_type = String, format = "datetime-utc")]
    pub created_at: DateTimeWithTimeZone,
}

impl UserPassword {
    pub fn from_model(model: user_password::Model) -> Self {
        Self {
            id: model.id,
            password: model.password,
            user_id: model.user_id,
            created_at: model.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema, ApiComponent, ToSchema)]
pub struct CreateUserPassword {
    pub password: String,
    pub user_id: Uuid,
    #[schema(value_type = String, format = "datetime-utc")]
    pub password_salt: String,
}

impl CreateUserPassword {
    pub fn to_active_model(&self) -> user_password::ActiveModel {
        user_password::ActiveModel {
            user_id: Set(self.user_id.clone()),
            password: Set(self.password.clone()),
            password_salt: Set(self.password_salt.clone()),
            ..Default::default()
        }
    }
}
