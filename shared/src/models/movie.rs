use apistos::ApiComponent;
use sea_orm::prelude::{ DateTimeWithTimeZone, Uuid };
use utoipa::ToSchema;
use schemars::JsonSchema;
use serde::{ Deserialize, Serialize };

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
pub struct CreateMovieRequest {
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: Option<String>,
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
