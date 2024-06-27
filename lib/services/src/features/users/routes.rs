pub use super::handlers;
use actix_web::{middleware::Compat, web};
use tracing_actix_web::TracingLogger;

pub fn user_route(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/users")
        .wrap(Compat::new(TracingLogger::default()))
        .service(handlers::get_user_by_id)
        .service(handlers::get_all_user)
        .service(handlers::update_user)
        .service(handlers::delete_user);

    conf.service(scope);
}
