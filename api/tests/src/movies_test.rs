#[cfg(test)]
mod test {
    use actix_web::{ http::StatusCode, test, web, App };
    use api_lib::{
        repositories::movie_repo::{ MovieRepository, MovieResult },
        services::movies::service,
    };
    use sqlx::types::Uuid;
    use shared::models::movie::{ CreateMovieRequest, Movie, UpdateMovieRequest };

    struct MockRepository;

    #[async_trait::async_trait]
    impl MovieRepository for MockRepository {
        async fn get_movies(&self) -> MovieResult<Vec<Movie>> {
            Ok(
                vec![Movie {
                    id: Uuid::new_v4(),
                    title: "Mock Movie".to_string(),
                    director: "Mock Director".to_string(),
                    year: 2021,
                    poster: Some("http://example.com/poster.jpg".to_string()),
                    created_at: None,
                    updated_at: None,
                }]
            )
        }

        async fn get_movie(&self, id: &Uuid) -> MovieResult<Movie> {
            Ok(Movie {
                id: *id,
                title: "Mock Movie".to_string(),
                director: "Mock Director".to_string(),
                year: 2021,
                poster: Some("http://example.com/poster.jpg".to_string()),
                created_at: None,
                updated_at: None,
            })
        }

        async fn create_movie(&self, _request: &CreateMovieRequest) -> MovieResult<Movie> {
            Ok(Movie {
                id: Uuid::new_v4(),
                title: "New Mock Movie".to_string(),
                director: "New Mock Director".to_string(),
                year: 2022,
                poster: Some("http://example.com/newposter.jpg".to_string()),
                created_at: None,
                updated_at: None,
            })
        }

        async fn update_movie(&self, _request: &UpdateMovieRequest) -> MovieResult<Movie> {
            Ok(Movie {
                id: Uuid::new_v4(),
                title: "Updated Mock Movie".to_string(),
                director: "Updated Mock Director".to_string(),
                year: 2023,
                poster: Some("http://example.com/updatedposter.jpg".to_string()),
                created_at: None,
                updated_at: None,
            })
        }

        async fn delete_movie(&self, _id: &Uuid) -> MovieResult<Uuid> {
            Ok(Uuid::new_v4())
        }
    }

    #[actix_rt::test]
    async fn test_get_all_movies() {
        let app = App::new()
            .app_data(web::Data::<Box<dyn MovieRepository>>::new(Box::new(MockRepository)))
            .configure(service::<api_lib::db::postgres::PostgresConnection>); // Ensure this is properly configured

        let mut app = test::init_service(app).await;

        let request = test::TestRequest::get().uri("/api/movies").to_request();
        let response = test::call_service(&mut app, request).await;

        print!("{:?}", response);

        assert!(response.status().is_success());
        assert_eq!(response.status(), StatusCode::OK);
    }

    // #[actix_rt::test]
    // async fn test_get_movie_by_id() {
    //     let app = App::new()
    //         .app_data(web::Data::<Box<dyn MovieRepository>>::new(Box::new(MockRepository)))
    //         .configure(service::<api_lib::db::postgres::PostgresConnection>);

    //     let movie_id = Uuid::new_v4();
    //     let mut app = test::init_service(app).await;
    //     let request = test::TestRequest
    //         ::get()
    //         .uri(&format!("/api/movies/{}", movie_id))
    //         .to_request();
    //     let response = test::call_service(&mut app, request).await;

    //     assert!(response.status().is_success());
    //     assert_eq!(response.status(), StatusCode::OK);
    // }

    // #[actix_rt::test]
    // async fn test_create_movie() {
    //     let app = App::new()
    //         .app_data(web::Data::<Box<dyn MovieRepository>>::new(Box::new(MockRepository)))
    //         .configure(service::<api_lib::db::postgres::PostgresConnection>);

    //     let mut app = test::init_service(app).await;
    //     let new_movie = CreateMovieRequest {
    //         title: "Test Movie".to_string(),
    //         director: "Test Director".to_string(),
    //         year: 2022,
    //         poster: Some("http://example.com/testposter.jpg".to_string()),
    //     };

    //     let request = test::TestRequest
    //         ::post()
    //         .uri("/api/movies")
    //         .set_json(&new_movie)
    //         .to_request();

    //     let response = test::call_service(&mut app, request).await;

    //     assert!(response.status().is_success());
    //     assert_eq!(response.status(), StatusCode::OK);
    // }

    // #[actix_rt::test]
    // async fn test_update_movie() {
    //     let app = App::new()
    //         .app_data(web::Data::<Box<dyn MovieRepository>>::new(Box::new(MockRepository)))
    //         .configure(service::<api_lib::db::postgres::PostgresConnection>);

    //     let mut app = test::init_service(app).await;
    //     let update_request = UpdateMovieRequest {
    //         id: Uuid::new_v4(),
    //         title: "Updated Test Movie".to_string(),
    //         director: "Updated Test Director".to_string(),
    //         year: 2023,
    //         poster: Some("http://example.com/updatedtestposter.jpg".to_string()),
    //     };

    //     let request = test::TestRequest
    //         ::put()
    //         .uri("/api/movies")
    //         .set_json(&update_request)
    //         .to_request();

    //     let response = test::call_service(&mut app, request).await;

    //     assert!(response.status().is_success());
    //     assert_eq!(response.status(), StatusCode::OK);
    // }

    // #[actix_rt::test]
    // async fn test_delete_movie() {
    //     let app = App::new()
    //         .app_data(web::Data::<Box<dyn MovieRepository>>::new(Box::new(MockRepository)))
    //         .configure(service::<api_lib::db::postgres::PostgresConnection>);

    //     let movie_id = Uuid::new_v4();
    //     let mut app = test::init_service(app).await;
    //     let request = test::TestRequest
    //         ::delete()
    //         .uri(&format!("/api/movies/{}", movie_id))
    //         .to_request();
    //     let response = test::call_service(&mut app, request).await;

    //     assert!(response.status().is_success());
    //     assert_eq!(response.status(), StatusCode::OK);
    // }
}
