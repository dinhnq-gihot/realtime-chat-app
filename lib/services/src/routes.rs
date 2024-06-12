use actix_web::web;
use super::features::*;

pub fn app_route(conf: &mut web::ServiceConfig) {
    auth::routes::auth_route(conf);
}
