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
    pub id: Uuid,
    pub title: String,
    pub director: String,
    pub year: i16,
    pub poster: Option<String>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
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
    pub id: Uuid,
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: Option<String>,
}
