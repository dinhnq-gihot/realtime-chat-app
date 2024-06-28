use chatapp_db::models::group::Group;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{features::users::types::UserResponseData, utils::general_response::GenericResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct RoomResponseData {
    pub id: Uuid,
    pub name: String,
}

impl From<Group> for RoomResponseData {
    fn from(value: Group) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRoomRequest {
    pub room_name: String,
    pub user_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddUserToRoomRequest {
    pub room_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateRoomRequest {
    pub new_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetRoomResponseData {
    pub room: RoomResponseData,
    pub users: Vec<UserResponseData>,
}

pub type CreateRoomResponse = GenericResponse<String>;
pub type GetRoomResponse = GenericResponse<GetRoomResponseData>;
pub type GetRoomsResponse = GenericResponse<Vec<RoomResponseData>>;
pub type UpdateRoomResponse = GenericResponse<RoomResponseData>;
pub type AddUserToRoomResponse = GenericResponse<String>;
pub type DeleteRoomResponse = GenericResponse<String>;
