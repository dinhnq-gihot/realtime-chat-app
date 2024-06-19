mod config;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::anyhow;
use chatapp_services::routes;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::EnvFilter;

#[get("/")]
async fn hello() -> impl Responder {
    tracing::info!("Hello path");
    HttpResponse::Ok().body("Hello world!")
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new("my_secret".to_string()))
            .service(hello)
            .configure(routes::app_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(|e| anyhow!(e.to_string()))
}
