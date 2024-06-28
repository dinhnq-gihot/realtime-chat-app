use std::sync::Arc;

use super::types::*;
use crate::{
    errors::{MyError, Result as CustomResult},
    features::users::types::UserResponseData,
};
use actix_web::{
    delete, get,
    http::StatusCode,
    patch, post,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use chatapp_db::{
    database::Database,
    repositories::{room::Rooms, user::Users},
};
use log::debug;
use uuid::Uuid;

use super::types::CreateRoomRequest;

#[post("")]
pub async fn create_room(
    req: HttpRequest,
    payload: Json<CreateRoomRequest>,
    db: Data<Arc<Database>>,
) -> CustomResult<HttpResponse> {
    debug!("{req:#?}");

    let room_repo = Rooms::new(Arc::clone(&db.into_inner()));
    let CreateRoomRequest {
        room_name,
        user_ids,
    } = payload.into_inner();

    let new_room = room_repo
        .create_room(room_name)
        .await
        .map_err(MyError::InternalError)?;

    for user_id in user_ids.into_iter() {
        room_repo
            .add_user_to_room(new_room.id, user_id)
            .await
            .map_err(MyError::InternalError)?;
    }

    Ok(HttpResponse::Ok()
        .status(StatusCode::CREATED)
        .json(CreateRoomResponse {
            msg: "success".into(),
            data: None,
        }))
}

#[get("/{id}")]
pub async fn get_room_by_id(
    req: HttpRequest,
    id: Path<Uuid>,
    db: Data<Arc<Database>>,
) -> CustomResult<HttpResponse> {
    debug!("{req:#?}");
    let db = db.into_inner();

    let room_repo = Rooms::new(Arc::clone(&db));
    let room = room_repo
        .get_room_by_id(id.into_inner())
        .await
        .map_err(MyError::InternalError)?;

    if room.id.is_nil() {
        return Err(MyError::NotFound);
    }

    let user_repo = Users::new(Arc::clone(&db));
    let users_in_room = user_repo
        .get_users_by_room(&room)
        .await
        .map_err(MyError::InternalError)?
        .into_iter()
        .map(UserResponseData::from)
        .collect::<Vec<UserResponseData>>();

    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(GetRoomResponse {
            msg: "success".into(),
            data: Some(GetRoomResponseData {
                room: room.into(),
                users: users_in_room,
            }),
        }))
}

#[get("/user/{id}")]
pub async fn get_rooms_by_user(
    req: HttpRequest,
    user_id: Path<Uuid>,
    db: Data<Arc<Database>>,
) -> CustomResult<HttpResponse> {
    debug!("{req:#?}");
    let db = db.into_inner();

    let user_repo = Users::new(Arc::clone(&db));
    let user = user_repo
        .get_user_by_id(user_id.into_inner())
        .await
        .map_err(MyError::InternalError)?;

    let room_repo = Rooms::new(Arc::clone(&db));
    let rooms = room_repo
        .get_rooms_by_user(&user)
        .await
        .map_err(MyError::InternalError)?
        .into_iter()
        .map(|r| r.into())
        .collect::<Vec<RoomResponseData>>();

    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(GetRoomsResponse {
            msg: "success".into(),
            data: Some(rooms),
        }))
}

#[post("/add-user")]
pub async fn add_user_to_room(
    req: HttpRequest,
    payload: Json<AddUserToRoomRequest>,
    db: Data<Arc<Database>>,
) -> CustomResult<HttpResponse> {
    debug!("{req:#?}");
    let AddUserToRoomRequest { room_id, user_id } = payload.into_inner();

    let room_repo = Rooms::new(Arc::clone(&db.into_inner()));
    room_repo
        .add_user_to_room(room_id, user_id)
        .await
        .map_err(MyError::InternalError)?;

    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(AddUserToRoomResponse {
            msg: "success".into(),
            data: None,
        }))
}

#[patch("/{id}")]
pub async fn update_room_name(
    req: HttpRequest,
    id: Path<Uuid>,
    payload: Json<UpdateRoomRequest>,
    db: Data<Arc<Database>>,
) -> CustomResult<HttpResponse> {
    debug!("{req:#?}");

    let UpdateRoomRequest { new_name } = payload.into_inner();
    let room_repo = Rooms::new(Arc::clone(&db.into_inner()));

    let ret = room_repo
        .update_name(id.into_inner(), Some(new_name))
        .await
        .map_err(MyError::InternalError)?;

    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(UpdateRoomResponse {
            msg: "success".into(),
            data: Some(RoomResponseData::from(ret)),
        }))
}

#[delete("/{id}")]
pub async fn delete_room(
    req: HttpRequest,
    id: Path<Uuid>,
    db: Data<Arc<Database>>,
) -> CustomResult<HttpResponse> {
    debug!("{req:#?}");

    let room_repo = Rooms::new(Arc::clone(&db.into_inner()));
    room_repo
        .delete(id.into_inner())
        .await
        .map_err(MyError::InternalError)?;

    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(DeleteRoomResponse {
            msg: "success".into(),
            data: None,
        }))
}
