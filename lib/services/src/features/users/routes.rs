use super::handlers;
use actix_web::web;

pub fn user_route(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/users")
        .service(handlers::get_user_by_id)
        .service(handlers::get_all_user)
        .service(handlers::update_user)
        .service(handlers::delete_user);

    conf.service(scope);
}
