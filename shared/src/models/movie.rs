use serde::{ Deserialize, Serialize };
use sqlx::{ prelude::FromRow, types::{ chrono::{ DateTime, Utc }, Uuid } };

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, FromRow)]
pub struct Movie {
    pub id: Uuid,
    pub title: String,
    pub director: String,
    #[sqlx(try_from = "i16")]
    pub year: u16,
    pub poster: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, FromRow)]
pub struct CreateMovieRequest {
    pub title: String,
    pub director: String,
    #[sqlx(try_from = "i16")]
    pub year: u16,
    pub poster: Option<String>,
}
