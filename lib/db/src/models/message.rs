use {
    crate::{
        models::{group::Group, user::User},
        schema::sql_types::MessageTypes,
    },
    chrono::NaiveDateTime,
    diesel::{
        deserialize::{FromSql, FromSqlRow},
        expression::AsExpression,
        pg::Pg,
        prelude::*,
        serialize::{IsNull, ToSql},
    },
    serde::{Deserialize, Serialize},
    std::io::Write,
    uuid::Uuid,
};

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = MessageTypes)]
pub enum MessageType {
    Text,
    Image,
    Video,
    File,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::Text
    }
}

impl ToSql<MessageTypes, Pg> for MessageType {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
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
            b"text" => Ok(MessageType::Text),
            b"image" => Ok(MessageType::Image),
            b"video" => Ok(MessageType::Video),
            b"file" => Ok(MessageType::File),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl From<String> for MessageType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "text" => Self::Text,
            "file" => Self::File,
            "image" => Self::Image,
            "video" => Self::Video,
            _ => Self::Text,
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
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub content: Option<String>,
    #[diesel(column_name = "type_")]
    pub r#type: Option<MessageType>,
    pub created_at: NaiveDateTime,
    pub edited_at: NaiveDateTime,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            user_id: Default::default(),
            group_id: Default::default(),
            content: Default::default(),
            r#type: Default::default(),
            created_at: Default::default(),
            edited_at: Default::default(),
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMessage<'a> {
    pub id: &'a Uuid,
    pub user_id: &'a Uuid,
    pub group_id: &'a Uuid,
    pub content: Option<&'a str>,
    #[diesel(column_name = "type_")]
    pub r#type: Option<&'a MessageType>,
    pub created_at: &'a NaiveDateTime,
}
