use {
    super::types::{
        CreateResponse, CreateUserRequest, DeleteResponse, GetAllResponse, GetResponse,
        UpdateResponse, UpdateUserRequest, UserResponseData,
    },
    crate::{
        errors::{MyError, Result as CustomResult},
        features::auth::types::Claims,
    },
    actix_web::{
        delete, get,
        http::StatusCode,
        patch, post,
        web::{Data, Json, Path},
        HttpMessage, HttpRequest, HttpResponse,
    },
    chatapp_db::{database::Database, models::user::NewUser, repositories::user::Users},
    std::sync::Arc,
    tracing::debug,
    uuid::Uuid,
};

#[post("/sign-up")]
pub async fn create_user(
    req: HttpRequest,
    payload: Json<CreateUserRequest>,
    db: Data<Database>,
) -> CustomResult<HttpResponse> {
    debug!("{req:#?}");

    let user_repo = Users::new(Arc::clone(&db.into_inner()));
    let _payload = payload.into_inner();

    let new_user = NewUser {
        name: &_payload.name,
        email: &_payload.email,
        avatar: _payload.avatar.as_deref(),
        id: &Uuid::new_v4(),
        password: &_payload.password,
    };

    let created_user = user_repo
        .create_user(new_user)
        .await
        .map_err(MyError::InternalError)?;

    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(CreateResponse {
            msg: "success".into(),
            data: Some(UserResponseData::from(created_user)),
        }))
}

#[get("/{id}")]
pub async fn get_user_by_id(
    req: HttpRequest,
    id: Path<Uuid>,
    db: Data<Database>,
) -> CustomResult<HttpResponse> {
    debug!("{req:#?}");

    let extension = req.extensions();
    let claims = (*extension).get::<Claims>();
    println!("{claims:#?}");

    let user_repo = Users::new(Arc::clone(&db.into_inner()));
    let user = user_repo
        .get_user_by_id(id.into_inner())
        .await
        .map_err(MyError::InternalError)?;

    Ok(HttpResponse::Ok().status(StatusCode::OK).json(GetResponse {
        msg: "success".into(),
        data: Some(UserResponseData::from(user)),
    }))
}

#[get("")]
pub async fn get_all_user(req: HttpRequest, db: Data<Database>) -> CustomResult<HttpResponse> {
    debug!("{req:#?}");
    println!("{req:#?}");

    let user_repo = Users::new(Arc::clone(&db.into_inner()));
    let all_users = user_repo
        .get_all_users()
        .await
        .map_err(MyError::InternalError)?
        .into_iter()
        .map(UserResponseData::from)
        .collect::<Vec<UserResponseData>>();

    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(GetAllResponse {
            msg: "success".into(),
            data: Some(all_users),
        }))
}

#[patch("/{id}")]
pub async fn update_user(
    req: HttpRequest,
    db: Data<Database>,
    id: Path<Uuid>,
    payload: Json<UpdateUserRequest>,
) -> CustomResult<HttpResponse> {
    debug!("{req:#?}");

    let user_repo = Users::new(Arc::clone(&db.into_inner()));
    let _payload = payload.into_inner();

    let updated_user = user_repo
        .update_user(
            id.into_inner(),
            _payload.name,
            _payload.email,
            _payload.avatar,
        )
        .await
        .map_err(MyError::InternalError)?;

    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(UpdateResponse {
            msg: "success".into(),
            data: Some(UserResponseData::from(updated_user)),
        }))
}

#[delete("/{id}")]
pub async fn delete_user(
    req: HttpRequest,
    db: Data<Database>,
    id: Path<Uuid>,
) -> CustomResult<HttpResponse> {
    debug!("{req:#?}");

    let user_repo = Users::new(Arc::clone(&db.into_inner()));
    user_repo
        .delete_user(id.into_inner())
        .await
        .map_err(MyError::InternalError)?;

    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(DeleteResponse {
            msg: "success".into(),
            data: None,
        }))
}
