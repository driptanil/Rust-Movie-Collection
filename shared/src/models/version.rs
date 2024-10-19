use apistos::ApiComponent;
use schemars::JsonSchema;
use utoipa::ToSchema;
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
pub struct Version {
    pub db: String,
}
