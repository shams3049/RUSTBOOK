use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware};
use actix_cors::Cors;
use serde::Serialize;
use std::io::Error;
use log::{info, debug, warn, error};

mod routes;
mod handlers;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    std::env::set_var("RUST_LOG", "info");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .configure(routes::init_routes)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")
    .map_err(|e| {
        error!("Failed to bind to address: {}", e);
        e
    })?
    .run()
    .await
    .map_err(|e| {
        error!("Server error: {}", e);
        e
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::test;
    use actix_web::{test, App};

    #[test]
    async fn test_get_data() {
        let app = test::init_service(
            App::new().configure(routes::init_routes)
        ).await;

        let req = test::TestRequest::get().uri("/api/data").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
