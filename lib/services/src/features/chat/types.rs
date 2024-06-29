use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct SendChatRequest {
    pub user_id: Uuid,
    pub room_id: Uuid,
    pub content: String,
    pub message_type: String,
}
