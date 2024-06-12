mod config;

use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::anyhow;
use chatapp_services::routes;
use tracing_actix_web::TracingLogger;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    chatapp_logger::init(None::<String>, true)?;
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
