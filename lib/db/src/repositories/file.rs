use anyhow::Result;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    database::Database,
    models::{
        file::{File, NewFile},
        group::Group,
        message::{Message, MessageType},
        user::User,
    },
    schema::{files, groups, messages, users},
};

#[derive(Debug)]
pub struct Files {
    db: Arc<Database>,
}

impl Files {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_file_by_id(&self, id: Uuid) -> Result<File> {
        let mut conn = self.db.get_connection().await;
        let res = files::table
            .find(id)
            .select(File::as_select())
            .first(&mut conn)
            .await;

        match res {
            Ok(ret) => Ok(ret),
            Err(e) => match e {
                diesel::NotFound => Ok(File::default()),
                _ => Err(e.into()),
            },
        }
    }

    pub async fn create_file(
        &self,
        message_id: Uuid,
        filename: String,
        file_path: String,
    ) -> Result<File> {
        let mut conn = self.db.get_connection().await;

        let new_file = NewFile {
            message_id: &message_id,
            filename: &filename,
            file_path: &file_path,
        };

        let ret = diesel::insert_into(files::table)
            .values(new_file)
            .returning(File::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn update_message(&self, id: Uuid, filename: String) -> Result<File> {
        let mut conn = self.db.get_connection().await;
        let mut file = self.get_file_by_id(id).await?;

        if file.message_id.is_nil() {
            return Ok(file);
        }
        file.filename = filename;

        let ret = diesel::update(files::table)
            .filter(files::message_id.eq(id))
            .set(file)
            .returning(File::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn delete_message(&self, id: Uuid) -> Result<()> {
        let mut conn = self.db.get_connection().await;
        diesel::delete(files::table)
            .filter(files::message_id.eq(id))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn get_files_in_room(&self, room_id: Uuid) -> Result<Vec<(File, Message)>> {
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

        let messages = messages::table
            .filter(messages::group_id.eq(room.id))
            .load::<Message>(&mut conn)
            .await?
            .into_iter()
            .filter(|v| v.r#type == Some(MessageType::File))
            .collect::<Vec<_>>();

        let mut files: Vec<File> = vec![];
        for m in messages.iter() {
            let f = files::table
                .find(m.id)
                .select(File::as_select())
                .get_result(&mut conn)
                .await?;

            files.push(f);
        }

        let ret = files
            .into_iter()
            .zip(messages.into_iter())
            .collect::<Vec<_>>();

        Ok(ret)
    }

    pub async fn get_messages_by_user(&self, user_id: Uuid) -> Result<Vec<(File, Message)>> {
        let mut conn = self.db.get_connection().await;
        let res = users::table
            .find(user_id)
            .select(User::as_select())
            .first(&mut conn)
            .await;

        let user: User = match res {
            Ok(room) => room,
            Err(e) => match e {
                diesel::NotFound => User::default(),
                _ => return Err(e.into()),
            },
        };

        let messages = messages::table
            .filter(messages::user_id.eq(user.id))
            .load::<Message>(&mut conn)
            .await?
            .into_iter()
            .filter(|v| v.r#type == Some(MessageType::File))
            .collect::<Vec<_>>();

        let mut files: Vec<File> = vec![];
        for m in messages.iter() {
            let f = files::table
                .find(m.id)
                .select(File::as_select())
                .get_result(&mut conn)
                .await?;

            files.push(f);
        }

        let ret = files
            .into_iter()
            .zip(messages.into_iter())
            .collect::<Vec<_>>();

        Ok(ret)
    }
}
