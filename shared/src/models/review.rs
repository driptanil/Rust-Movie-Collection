use apistos::ApiComponent;
use sea_orm::prelude::{ DateTimeWithTimeZone, Uuid, Decimal };
use utoipa::ToSchema;
use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };
use sea_orm::{ ActiveModelBehavior, ActiveValue };
use num_traits::cast::ToPrimitive; // Import ToPrimitive trait
use num_traits::FromPrimitive; // Import FromPrimitive trait
use crate::entities::{ movie, review };

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Default,
    JsonSchema,
    ApiComponent,
    ToSchema
)]
pub struct Review {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub rating: f32, // Use f32 for the rating field
    pub comment: String,
    #[schema(value_type = String, format = "datetime-utc")]
    pub created_at: DateTimeWithTimeZone,
    #[schema(value_type = String, format = "datetime-utc")]
    pub updated_at: DateTimeWithTimeZone,
    #[schema(value_type = String, format = "uuid")]
    pub user_id: Uuid,
    #[schema(value_type = String, format = "uuid")]
    pub movie_id: Uuid,
}

impl Review {
    pub fn from_model(model: review::Model) -> Self {
        Self {
            id: model.id,
            rating: model.rating.to_f32().unwrap_or_default(),
            comment: model.comment,
            created_at: model.created_at,
            updated_at: model.updated_at,
            user_id: model.user_id,
            movie_id: model.movie_id,
        }
    }

    pub fn from_list_models(models: Vec<review::Model>) -> Vec<Self> {
        models
            .into_iter()
            .map(|model| Self::from_model(model))
            .collect()
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    Default,
    JsonSchema,
    ToSchema
)]
pub struct CreateReviewRequest {
    pub rating: f32,
    pub comment: String,
    pub user_id: Uuid,
    pub movie_id: Uuid,
}

impl CreateReviewRequest {
    pub fn to_active_model(&self) -> review::ActiveModel {
        review::ActiveModel {
            rating: ActiveValue::Set(Decimal::from_f32(self.rating).unwrap_or_default()),
            comment: ActiveValue::Set(self.comment.clone()),
            user_id: ActiveValue::Set(self.user_id),
            movie_id: ActiveValue::Set(self.movie_id),
            ..Default::default()
        }
    }
}
