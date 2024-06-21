use actix_web::web;
use tracing::{instrument, Level};
use tracing_actix_web::TracingLogger;

use super::handlers;

#[instrument(skip_all, level = Level::INFO)]
pub fn user_route(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/users")
        .wrap(TracingLogger::default())
        .service(handlers::get_user_by_id)
        .service(handlers::get_all_user)
        .service(handlers::update_user)
        .service(handlers::delete_user);

    conf.service(scope);
}
