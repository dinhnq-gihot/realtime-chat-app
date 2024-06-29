use anyhow::Result;
use chrono::Local;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    database::Database,
    models::{
        group::Group,
        message::{Message, MessageType, NewMessage},
        user::User,
    },
    schema::{groups, messages, users},
};

#[derive(Debug)]
pub struct Messages {
    db: Arc<Database>,
}

impl Messages {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_message_by_id(&self, id: Uuid) -> Result<Message> {
        let mut conn = self.db.get_connection().await;
        let res = messages::table
            .find(id)
            .select(Message::as_select())
            .first(&mut conn)
            .await;

        match res {
            Ok(ret) => Ok(ret),
            Err(e) => match e {
                diesel::NotFound => Ok(Message::default()),
                _ => Err(e.into()),
            },
        }
    }

    pub async fn create_message(
        &self,
        content: Option<String>,
        user_id: Uuid,
        room_id: Uuid,
        r#type: Option<MessageType>,
    ) -> Result<Message> {
        let mut conn = self.db.get_connection().await;
        let _type = Some(r#type.unwrap_or_default());

        let new_message = NewMessage {
            id: &Uuid::new_v4(),
            user_id: &user_id,
            group_id: &room_id,
            content: content.as_deref(),
            r#type: _type.as_ref(),
            created_at: &Local::now().naive_utc(),
        };

        let ret = diesel::insert_into(messages::table)
            .values(new_message)
            .returning(Message::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn update_message(&self, id: Uuid, content: Option<String>) -> Result<Message> {
        let mut conn = self.db.get_connection().await;
        let mut message = self.get_message_by_id(id).await?;

        if message.id.is_nil() {
            return Ok(message);
        }
        message.content = content;

        let ret = diesel::update(messages::table)
            .filter(messages::id.eq(id))
            .set(message)
            .returning(Message::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn delete_message(&self, id: Uuid) -> Result<()> {
        let mut conn = self.db.get_connection().await;
        diesel::delete(messages::table)
            .filter(messages::id.eq(id))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn get_messages_in_room(&self, room_id: Uuid) -> Result<Vec<Message>> {
        let mut conn = self.db.get_connection().await;
        let res = groups::table
            .find(room_id)
            .select(Group::as_select())
            .first(&mut conn)
            .await;

        let room: Group = match res {
            Ok(room) => room,
            Err(e) => match e {
                diesel::NotFound => Group::default(),
                _ => return Err(e.into()),
            },
        };

        let ret = messages::table
            .filter(messages::group_id.eq(room.id))
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get_messages_by_user(&self, user_id: Uuid) -> Result<Vec<Message>> {
        let mut conn = self.db.get_connection().await;
        let res = users::table
            .find(user_id)
            .select(User::as_select())
            .first(&mut conn)
            .await;

        let user: User = match res {
            Ok(user) => user,
            Err(e) => match e {
                diesel::NotFound => User::default(),
                _ => return Err(e.into()),
            },
        };

        let ret = messages::table
            .filter(messages::user_id.eq(user.id))
            .load(&mut conn)
            .await?;

        Ok(ret)
    }
}
