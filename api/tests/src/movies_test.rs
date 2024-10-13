#[cfg(test)]
mod test {
    use actix_web::http::StatusCode;
    use actix_web::{ test, web, App };
    use sqlx::types::{ chrono, Uuid };
    use api_lib::services::movies::{ self, service };
    use api_lib::repositories::movie_repo::MovieRepository;
    use shared::models::movie::{ CreateMovieRequest, Movie, UpdateMovieRequest };
    use std::sync::Arc;

    // Mock implementation of MovieRepository for testing
    struct MockMovieRepository;

    #[async_trait::async_trait]
    impl MovieRepository for MockMovieRepository {
        async fn get_movies(&self) -> Result<Vec<Movie>, String> {
            Ok(
                vec![Movie {
                    id: Uuid::new_v4(),
                    title: "Mock Movie".to_string(),
                    director: "Director Name".to_string(),
                    year: 2021,
                    poster: Some("http://example.com/poster.jpg".to_string()),
                    created_at: Some(chrono::Utc::now()),
                    updated_at: Some(chrono::Utc::now()),
                }]
            )
        }

        async fn get_movie(&self, movie_id: &Uuid) -> Result<Movie, String> {
            todo!()
        }

        async fn create_movie(&self, movie: &CreateMovieRequest) -> Result<Movie, String> {
            todo!()
        }

        async fn update_movie(&self, movie: &UpdateMovieRequest) -> Result<Movie, String> {
            todo!()
        }

        async fn delete_movie(&self, movie_id: &Uuid) -> Result<Uuid, String> {
            todo!()
        }

        // Implement other methods as needed for your tests
    }

    #[actix_rt::test]
    async fn get_all_movies_check_works() {
        let app = App::new().configure(service);

        let mut app = actix_web::test::init_service(app).await;

        let request = actix_web::test::TestRequest::get().uri("/movies").to_request();

        let response = test::call_service(&app, request).await;

        print!("{:?}", response);

        assert!(response.status().is_success());
        assert_eq!(response.status(), StatusCode::OK);

        // // Optionally, check the response body if needed
        // let body = test::read_body(resp).await;
        // assert!(!body.is_empty()); // Verify that the body is not empty
    }
}
