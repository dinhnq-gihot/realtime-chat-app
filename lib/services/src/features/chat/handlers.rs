use std::sync::Arc;

use super::types::SendChatRequest;
use crate::errors::{MyError, Result as CustomResult};
use actix_web::{
    http::StatusCode,
    post,
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use chatapp_db::{database::Database, repositories::message::Messages};
use serde_json::json;

#[post("")]
pub async fn send_message(
    req: HttpRequest,
    payload: Json<SendChatRequest>,
    db: web::Data<Arc<Database>>,
) -> CustomResult<HttpResponse> {
    let db = db.into_inner();
    let message_repo = Messages::new(Arc::clone(&db));

    let SendChatRequest {
        user_id,
        room_id,
        content,
        message_type,
    } = payload.into_inner();

    let new_message = message_repo
        .create_message(Some(content), user_id, room_id, Some(message_type.into()))
        .await
        .map_err(MyError::InternalError);

    // TODO: create file

    Ok(HttpResponse::Ok().status(StatusCode::OK).json(json!({
        "msg": "success"
    })))
}
