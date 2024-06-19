use {
    super::{user::User, group::Group},
    diesel::prelude::*
};

#[derive(Debug, Insertable, Selectable, Associations, Clone)]
#[diesel(table_name = crate::schema::users_groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(primary_key(user_id, group_id))]
pub struct UserGroup {
    pub user_id: uuid::Uuid,
    pub group_id: uuid::Uuid
}