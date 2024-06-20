use {
    super::{
        repositories::Users,
        types::{
            CreateResponse, CreateUserRequest, DeleteResponse, GetAllResponse, GetResponse,
            UpdateResponse, UpdateUserRequest, UserResponseData,
        },
    },
    actix_web::{
        delete, get,
        http::StatusCode,
        patch, post,
        web::{Data, Json, Path},
        HttpRequest, HttpResponse,
    },
    chatapp_db::database::Database,
    std::sync::Arc,
    tracing::debug,
    uuid::Uuid,
};

#[post("/sign-up")]
pub async fn create_user(
    req: HttpRequest,
    payload: Json<CreateUserRequest>,
    db: Data<Database>,
) -> HttpResponse {
    debug!("{req:#?}");

    let user_repo = Users::new(Arc::clone(&db.into_inner()));
    let created_user = user_repo.create_user(payload.into_inner()).await.unwrap();

    HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(UserResponseData::from(created_user))
}

#[get("/{id}")]
pub async fn get_user_by_id(req: HttpRequest, id: Path<Uuid>, db: Data<Database>) -> HttpResponse {
    debug!("{req:#?}");

    let user_repo = Users::new(Arc::clone(&db.into_inner()));
    let user = user_repo.get_user_by_id(id.into_inner()).await.unwrap();

    HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(UserResponseData::from(user))
}

#[get("/")]
pub async fn get_all_user(req: HttpRequest, db: Data<Database>) -> HttpResponse {
    debug!("{req:#?}");
    let user_repo = Users::new(Arc::clone(&db.into_inner()));
    let all_users = user_repo
        .get_all_users()
        .await
        .unwrap()
        .into_iter()
        .map(|user| UserResponseData::from(user))
        .collect::<Vec<UserResponseData>>();

    HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(GetAllResponse {
            msg: "success".into(),
            data: Some(all_users),
        })
}

#[patch("/{id}")]
pub async fn update_user(
    req: HttpRequest,
    db: Data<Database>,
    id: Path<Uuid>,
    payload: Json<UpdateUserRequest>,
) -> HttpResponse {
    debug!("{req:#?}");

    let user_repo = Users::new(Arc::clone(&db.into_inner()));
    let updated_user = user_repo
        .update_user(id.into_inner(), payload.into_inner())
        .await
        .unwrap();

    HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(UpdateResponse {
            msg: "success".into(),
            data: Some(UserResponseData::from(updated_user)),
        })
}

#[delete("/{id}")]
pub async fn delete_user(req: HttpRequest, db: Data<Database>, id: Path<Uuid>) -> HttpResponse {
    debug!("{req:#?}");

    let user_repo = Users::new(Arc::clone(&db.into_inner()));
    user_repo.delete_user(id.into_inner()).await.unwrap();

    HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(DeleteResponse {
            msg: "success".into(),
            data: None,
        })
}
