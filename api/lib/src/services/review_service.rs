use shared::{ entities::review, models::review::{ CreateReviewRequest, Review } };
use sea_orm::prelude::Uuid;
use std::sync::Arc;
use crate::{
    repositories::review_repository::ReviewRepository,
    utils::error::{ ApiError, ApiResult },
};

#[async_trait::async_trait]
pub trait ReviewService: Send + Sync + 'static {
    async fn get_review_by_movie_id(&self, movie_id: Uuid) -> ApiResult<Vec<Review>>;
    async fn create_movie(&self, review: CreateReviewRequest) -> ApiResult<Review>;
}

pub struct ReviewServiceImpl {
    repo: Arc<dyn ReviewRepository>,
}

impl ReviewServiceImpl {
    pub fn new(repo: Arc<dyn ReviewRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl ReviewService for ReviewServiceImpl {
    async fn get_review_by_movie_id(&self, movie_id: Uuid) -> ApiResult<Vec<Review>> {
        let response = self.repo
            .get_review_by_movie_id(movie_id).await
            .map_err(|e| { ApiError::InternalServer(format!("Database error: {:?}", e)) })?;
        Ok(Review::from_list_models(response))
    }

    async fn create_movie(&self, review: CreateReviewRequest) -> ApiResult<Review> {
        let new_review = review.to_active_model();

        let inserted_review = self.repo
            .create_review(new_review).await
            .map_err(|e| { ApiError::InternalServer("Database error ):".to_owned()) })?;

        Ok(Review::from_model(inserted_review))
    }
}
