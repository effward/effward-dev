use actix_web::{HttpResponse, Responder, http::header::ContentType};

pub async fn health() -> impl Responder {
    println!("Healthcheck request received.");

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("healthy")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test, web, App,
    };

    #[actix_web::test]
    async fn test_manual_hello_ok() {
        let request = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();

        let response = health().await.respond_to(&request);
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_hello_get() {
        let app = test::init_service(App::new().route("/", web::get().to(health))).await;
        let request = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();

        let response = test::call_service(&app, request).await;
        assert!(response.status().is_success());
    }
}
