use actix_web::web;
use tracing::{instrument, Level};

use super::handler;

#[instrument(skip_all, level = Level::INFO)]
pub fn auth_route(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(handler::login)
        .service(handler::protected);
    conf.service(scope);
}
