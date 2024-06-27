use {
    super::{group::Group, user::User},
    crate::schema::users_groups,
    diesel::prelude::*,
};

#[derive(Debug, Identifiable, Selectable, Associations, Clone)]
#[diesel(table_name = crate::schema::users_groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(primary_key(user_id, group_id))]
pub struct UserGroup {
    pub user_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users_groups)]
pub struct NewUserGroup<'a> {
    pub user_id: &'a uuid::Uuid,
    pub group_id: &'a uuid::Uuid,
}
