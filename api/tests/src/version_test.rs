// #[cfg(test)]
// mod test {
//     use actix_web::{http::StatusCode, App};
//     use sqlx::{PgPool, Executor};
//     use actix_web::web;
//     use api_lib::services::version::{get, service}; // Adjust the path as necessary

//     #[actix_rt::test]
//     async fn test_get_version_success() {
//         // Setup an in-memory database for testing
//         let database_url = "postgres://username:password@localhost/test_db";
//         let pool = PgPool::connect(database_url).await.unwrap();
        
//         // Create the test database schema
//         pool.execute("CREATE TABLE IF NOT EXISTS version (id SERIAL PRIMARY KEY, version VARCHAR(255));").await.unwrap();
//         pool.execute("INSERT INTO version (version) VALUES ('PostgreSQL 14.2');").await.unwrap();

//         // Create the Actix Web app
//         let app = App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .configure(service);

//         // Create a test request
//         let request = actix_web::test::TestRequest::get()
//             .uri("/version")
//             .to_request();

//         // Call the service
//         let mut app = actix_web::test::init_service(app).await;
//         let response = actix_web::test::call_service(&mut app, request).await;

//         // Assert the response
//         assert!(response.status().is_success());
//         assert_eq!(response.status(), StatusCode::OK);
        
//         let body = actix_web::test::read_body_string(response).await;
//         assert_eq!(body, "PostgreSQL 14.2");
//     }

//     #[actix_rt::test]
//     async fn test_get_version_failure() {
//         // Setup an in-memory database for testing
//         let database_url = "postgres://username:password@localhost/test_db";
//         let pool = PgPool::connect(database_url).await.unwrap();

//         // Create the Actix Web app with a bad query
//         let app = App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .configure(service);

//         // Create a test request
//         let request = actix_web::test::TestRequest::get()
//             .uri("/version")
//             .to_request();

//         // Call the service
//         let mut app = actix_web::test::init_service(app).await;
//         let response = actix_web::test::call_service(&mut app, request).await;

//         // Assert the response
//         assert!(response.status().is_client_error());
//         assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        
//         let body = actix_web::test::read_body_string(response).await;
//         assert!(body.contains("Failed to fetch database version"));
//     }
// }
