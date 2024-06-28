mod config;

use std::sync::Arc;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use anyhow::anyhow;
use chatapp_db::database::Database;
use chatapp_services::routes;

#[get("/")]
async fn hello() -> impl Responder {
    tracing::info!("Hello path");
    HttpResponse::Ok().body("Hello world!")
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    chatapp_logger::init(None, false)?;

    chatapp_logger::info!("Starting HTTP server at http://localhost:8080");

    let db = Database::new("postgresql://chatapp:123@localhost:15432/chatapp".into()).await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(Arc::new(db.clone())))
            .service(hello)
            .configure(routes::app_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(|e| anyhow!(e.to_string()))
}
