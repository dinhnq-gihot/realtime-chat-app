use crate::errors::{MyError, Result as CustomResult};
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use chatapp_db::database::Database;
use log::debug;

use super::types::CreateRoomRequest;

pub async fn create_room(
    req: HttpRequest,
    payload: Json<CreateRoomRequest>,
    db: Data<Database>,
) -> CustomResult<HttpResponse> {
    debug!("{req:#?}");

    // let group_repo = Group

    Ok(HttpResponse::Ok().finish())
}
