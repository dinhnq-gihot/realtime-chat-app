pub use super::handlers;
use actix_web::web;

pub fn room_route(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/rooms")
        .service(handlers::create_room)
        .service(handlers::add_user_to_room)
        .service(handlers::delete_room)
        .service(handlers::get_room_by_id)
        .service(handlers::get_rooms_by_user)
        .service(handlers::update_room_name);

    conf.service(scope);
}
