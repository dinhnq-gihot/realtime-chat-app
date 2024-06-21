use {actix_web::web, super::handlers, crate::features::users::handlers as user_handlers};
use tracing::{instrument, Level};

#[instrument(skip_all, level = Level::INFO)]
pub fn auth_route(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(handlers::login)
        .service(handlers::protected)
        .service(user_handlers::create_user);

    conf.service(scope);
}
