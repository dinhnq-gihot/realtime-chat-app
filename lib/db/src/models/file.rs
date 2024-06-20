use {diesel::prelude::*, crate::models::message::Message};

#[derive(Debug, Identifiable, AsChangeset, Queryable, Selectable, Associations, PartialEq, Clone)]
#[diesel(table_name = crate::schema::files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Message, foreign_key = message_id))]
#[diesel(primary_key(message_id))]
pub struct File {
    pub message_id: uuid::Uuid,
    pub filename: String 
}


#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewFile<'a> {
    pub message_id: &'a uuid::Uuid,
    pub filename: &'a str
}
