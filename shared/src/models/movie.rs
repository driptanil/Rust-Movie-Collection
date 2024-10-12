use sqlx::types::{ Uuid, chrono::{ DateTime, Utc } };

pub struct Movie {
    pub id: Uuid,
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct CreateMovieRequest {
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: Option<String>,
}
