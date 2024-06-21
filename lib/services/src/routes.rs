use super::features::*;
use actix_web::web;

pub fn app_route(conf: &mut web::ServiceConfig) {
    users::routes::user_route(conf);
    auth::routes::auth_route(conf);
}
