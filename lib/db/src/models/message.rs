use {
    crate::{
        models::{group::Group, user::User},
        schema::sql_types::MessageTypes,
    }, chrono::{DateTime, Local}, diesel::{deserialize::{FromSql, FromSqlRow}, expression::AsExpression, pg::Pg, prelude::*, serialize::{IsNull, ToSql}}, serde::{Deserialize, Serialize}, std::io::Write, uuid::Uuid
};

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = MessageTypes)]
pub enum MessageType {
    Text,
    Image,
    Video,
    File,
}

impl ToSql<MessageTypes, Pg> for MessageType {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            MessageType::File => out.write_all(b"file")?,
            MessageType::Image => out.write_all(b"image")?,
            MessageType::Video => out.write_all(b"video")?,
            MessageType::Text => out.write_all(b"text")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<MessageTypes, Pg> for MessageType {
    fn from_sql(bytes: diesel::pg::PgValue) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Text" => Ok(MessageType::Text),
            b"Image" => Ok(MessageType::Image),
            b"Video" => Ok(MessageType::Video),
            b"File" => Ok(MessageType::File),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Queryable, Identifiable, AsChangeset, Selectable, Associations)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Group, foreign_key = group_id))]
pub struct Message {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub content: Option<String>,
    #[diesel(column_name = "type_")]
    pub r#type: Option<MessageType>,
    pub created_at: Option<DateTime<Local>>,
}
