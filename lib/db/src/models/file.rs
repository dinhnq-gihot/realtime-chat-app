use {crate::models::message::Message, diesel::prelude::*, uuid::Uuid};

#[derive(
    Debug, Identifiable, AsChangeset, Queryable, Selectable, Associations, PartialEq, Clone,
)]
#[diesel(table_name = crate::schema::files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Message, foreign_key = message_id))]
#[diesel(primary_key(message_id))]
pub struct File {
    pub message_id: uuid::Uuid,
    pub filename: String,
    pub file_path: String,
}

impl Default for File {
    fn default() -> Self {
        Self {
            message_id: Uuid::nil(),
            filename: Default::default(),
            file_path: Default::default(),
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewFile<'a> {
    pub message_id: &'a uuid::Uuid,
    pub filename: &'a str,
    pub file_path: &'a str,
}
