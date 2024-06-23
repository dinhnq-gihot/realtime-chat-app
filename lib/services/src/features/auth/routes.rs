use {super::handlers, crate::features::users::handlers as user_handlers, actix_web::web};

pub fn auth_route(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(handlers::login)
        .service(handlers::protected)
        .service(user_handlers::create_user);

    conf.service(scope);
}
