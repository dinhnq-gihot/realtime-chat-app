use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::users::types::UserResponseData;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub user_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoomResponseData {
    pub id: Uuid,
    pub users: Vec<UserResponseData>,
}
