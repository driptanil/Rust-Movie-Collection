#[cfg(test)]
mod test {
    use actix_web::{ http::StatusCode, App };
    use api_lib::services::health_service::{ get_health, API_VERSION };
    use api_lib::routers::health_router::router;

    #[actix_rt::test]
    async fn health_check_works() {
        let response = get_health().await;

        assert!(response.status().is_success());
        assert_eq!(response.status(), StatusCode::OK);

        let data = response
            .headers()
            .get("version")
            .and_then(|v| v.to_str().ok());

        assert_eq!(data, Some(API_VERSION));
    }

    #[actix_rt::test]
    async fn health_check_works_endpoint() {
        let app = App::new().configure(router);

        let mut app = actix_web::test::init_service(app).await;

        let request = actix_web::test::TestRequest::get().uri("/health").to_request();

        let response = actix_web::test::call_service(&mut app, request).await;

        assert!(response.status().is_success());
        assert_eq!(response.status(), StatusCode::OK);

        let data = response
            .headers()
            .get("version")
            .and_then(|v| v.to_str().ok());

        assert_eq!(data, Some(API_VERSION))
    }
}
