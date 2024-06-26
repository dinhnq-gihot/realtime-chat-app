use {
    crate::features::{room::types::RoomResponseData, users::types::UserResponseData},
    serde::{Deserialize, Serialize},
    utoipa::ToSchema,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[aliases(
    // user handlers responses
    GetResponse = GenericResponse<UserResponseData>,
    GetAllResponse = GenericResponse<Vec<UserResponseData>>,
    CreateResponse = GenericResponse<UserResponseData>,
    UpdateResponse = GenericResponse<UserResponseData>,
    DeleteResponse = GenericResponse<String>,

    // auth handlers responses
    LoginResponse = GenericResponse<String>,

    // room handlers responses
    CreateRoomResponse = GenericResponse<RoomResponseData>,
    GetRoomResponse = GenericResponse<RoomResponseData>
)]
pub struct GenericResponse<U> {
    pub msg: String,
    pub data: Option<U>,
}
