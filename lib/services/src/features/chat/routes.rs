pub use super::handlers;
use actix_web::web;

pub fn chat_route(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/chat").service(handlers::send_message);

    conf.service(scope);
}
