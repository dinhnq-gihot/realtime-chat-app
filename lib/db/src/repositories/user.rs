use {
    crate::{
        database::Database,
        models::user::{NewUser, User},
        schema::users,
    },
    anyhow::{anyhow, Result},
    diesel::{delete, insert_into, prelude::*, update},
    diesel_async::RunQueryDsl,
    std::sync::Arc,
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

    pub async fn create_user(&self, new_user: NewUser<'_>) -> Result<User> {
        let mut conn = self.db.get_connection().await;

        insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        name: Option<String>,
        email: Option<String>,
        avatar: Option<String>,
    ) -> Result<User> {
        let mut conn = self.db.get_connection().await;
        let mut existed_user: User = users::table
            .filter(users::id.eq(id))
            .select(User::as_select())
            .first(&mut conn)
            .await?;

        if name.is_some() {
            existed_user.name = name.unwrap();
        }
        if email.is_some() {
            existed_user.email = email.unwrap();
        }
        existed_user.avatar = avatar;

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
            .find(id)
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

    pub async fn login(&self, email: String, password: String) -> Result<User> {
        let mut conn = self.db.get_connection().await;
        users::table
            .filter(users::email.eq(email))
            .filter(users::password.eq(password))
            .select(User::as_select())
            .first(&mut conn)
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
