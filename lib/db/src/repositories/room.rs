use crate::{
    database::Database,
    models::{
        group::{self, Group, NewGroup},
        relationships::{NewUserGroup, UserGroup},
        user::User,
    },
    schema::{groups, users, users_groups},
};
use anyhow::Result;
use diesel::{insert_into, prelude::*};
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use uuid::Uuid;

pub struct Rooms {
    db: Arc<Database>,
}

impl Rooms {
    pub fn new(&self, db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn add_user_to_room(&self, room_id: Uuid, user_id: Uuid) -> Result<()> {
        let new_room_user = NewUserGroup {
            user_id: &user_id,
            group_id: &room_id,
        };

        let mut conn = self.db.get_connection().await;
        insert_into(users_groups::table)
            .values(new_room_user)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn create_room(&self, name: String) -> Result<Group> {
        let new_room = NewGroup {
            id: &Uuid::new_v4(),
            name: &name,
        };

        let mut conn = self.db.get_connection().await;
        let room: Group = insert_into(groups::table)
            .values(new_room)
            .returning(Group::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(room)
    }

    pub async fn get_room_by_id(&self, id: Uuid) -> Result<Group> {
        let mut conn = self.db.get_connection().await;
        let ret = groups::table
            .find(id)
            .select(Group::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get_room_by_user_id(&self, user: &User) -> Result<Vec<Group>> {
        let mut conn = self.db.get_connection().await;

        let groups = UserGroup::belonging_to(user)
            .inner_join(groups::table)
            .select(Group::as_select())
            .load(&mut conn)
            .await?;

        Ok(groups)
    }

    pub async fn update_name(&self, name: Option<String>) -> Result<Group> {
        let mut conn = self.db.get_connection().await;

        Ok(())
    }
}
