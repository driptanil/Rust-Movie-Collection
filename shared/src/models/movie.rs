use apistos::ApiComponent;
use sea_orm::{ prelude::{ DateTimeWithTimeZone, Uuid }, Set };
use utoipa::ToSchema;
use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };
use sea_orm::{ ActiveModelBehavior, ActiveValue };

use crate::entities::movie;

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
pub struct Movie {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub title: String,
    pub director: String,
    pub year: i16,
    pub poster: Option<String>,
    #[schema(value_type = String, format = "datetime-utc")]
    pub created_at: DateTimeWithTimeZone,
    #[schema(value_type = String, format = "datetime-utc")]
    pub updated_at: DateTimeWithTimeZone,
}

impl Movie {
    pub fn from_model(model: movie::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            director: model.director,
            year: model.year as i16,
            poster: model.poster,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }

    pub fn from_list_models(models: Vec<movie::Model>) -> Vec<Self> {
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
    Eq,
    PartialOrd,
    Ord,
    Default,
    JsonSchema,
    ToSchema
)]
pub struct CreateMovieRequest {
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: Option<String>,
}

impl CreateMovieRequest {
    pub fn to_active_model(&self) -> movie::ActiveModel {
        movie::ActiveModel {
            title: Set(self.title.clone()),
            director: Set(self.director.clone()),
            year: Set(self.year as i16),
            poster: Set(self.poster.clone()),
            ..Default::default()
        }
    }
}

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
pub struct UpdateMovieRequest {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: Option<String>,
}

impl UpdateMovieRequest {
    pub fn to_active_model(&self) -> movie::ActiveModel {
        movie::ActiveModel {
            id: ActiveValue::set(self.id),
            title: Set(self.title.clone()),
            director: Set(self.director.clone()),
            year: Set(self.year as i16),
            poster: Set(self.poster.clone()),
            ..Default::default()
        }
    }
}
