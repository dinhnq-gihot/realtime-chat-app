use {
    chatapp_db::models::user::User as UserModel,
    serde::{Deserialize, Serialize},
    utoipa::ToSchema,
    uuid::Uuid,
};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[aliases(
    GetResponse = GenericResponse<UserResponseData>,
    GetAllResponse = GenericResponse<Vec<UserResponseData>>,
    CreateResponse = GenericResponse<UserResponseData>,
    UpdateResponse = GenericResponse<UserResponseData>,
    DeleteResponse = GenericResponse<String>
)]
pub struct GenericResponse<U> {
    pub msg: String,
    pub data: Option<U>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseData {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
    pub is_online: bool,
}

// impl<'a> From<&'a CreateUserRequest> for NewUserModel<'a> {
//     fn from(value: &'a CreateUserRequest) -> Self {
//         Self {
//             name: &value.name,
//             email: &value.email,
//             password: &value.password,
//             avatar: value.avatar.as_deref(),
//         }
//     }
// }

impl From<UserModel> for UserResponseData {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            avatar: value.avatar,
            is_online: value.is_online.unwrap_or(false),
        }
    }
}
