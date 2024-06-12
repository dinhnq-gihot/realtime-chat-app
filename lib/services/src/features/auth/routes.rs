use actix_web::web;

use super::handler;

pub fn auth_route(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(handler::login)
        .service(handler::protected);
    conf.service(scope);
}
