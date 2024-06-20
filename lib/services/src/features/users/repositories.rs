use {
    super::types::{CreateUserRequest, UpdateUserRequest},
    anyhow::{anyhow, Result},
    chatapp_db::{
        database::Database,
        models::user::{NewUser, User},
        schema::users,
    },
    diesel::{delete, insert_into, prelude::*, update},
    diesel_async::RunQueryDsl,
    std::{borrow::Borrow, sync::Arc},
    uuid::Uuid,
};

#[derive(Debug)]
pub struct Users {
    pub db: Arc<Database>,
}

impl Users {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, new_user: CreateUserRequest) -> Result<User> {
        let mut conn = self.db.get_connection().await;

        let new_user: NewUser = new_user.borrow().into();

        insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn update_user(&self, id: Uuid, update_user: UpdateUserRequest) -> Result<User> {
        let mut conn = self.db.get_connection().await;
        let mut existed_user: User = users::table
            .filter(users::id.eq(id))
            .select(User::as_select())
            .first(&mut conn)
            .await?;

        if update_user.name.is_some() {
            existed_user.name = update_user.name.unwrap();
        }
        if update_user.email.is_some() {
            existed_user.email = update_user.email.unwrap();
        }
        existed_user.avatar = update_user.avatar;

        update(users::table.filter(users::id.eq(id)))
            .set(existed_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<User> {
        let mut conn = self.db.get_connection().await;
        users::table
            .filter(users::id.eq(id))
            .select(User::as_select())
            .first(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let mut conn = self.db.get_connection().await;
        users::table
            .load(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<()> {
        let mut conn = self.db.get_connection().await;
        delete(users::table.filter(users::id.eq(id)))
            .execute(&mut conn)
            .await?;
        Ok(())
    }
}
