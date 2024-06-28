use anyhow::Result;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use uuid::Uuid;

use crate::{database::Database, models::message::Message, schema::messages};

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
}
