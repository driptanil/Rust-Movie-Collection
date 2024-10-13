use actix_web::{ web, HttpResponse };

const API_VERSION: &str = "0.0.1";

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

#[tracing::instrument]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().append_header(("version", API_VERSION)).finish()
}

#[cfg(test)]
mod test {
    use actix_web::{ http::StatusCode, App };

    use super::*;

    #[actix_rt::test]
    async fn health_check_works() {
        let response = health().await;

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
        let app = App::new().configure(service);

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
