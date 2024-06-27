use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Identifiable, AsChangeset, Selectable, Queryable, PartialEq, Clone)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: uuid::Uuid,
    pub name: String,
}

impl Default for Group {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            name: "".into(),
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGroup<'a> {
    pub id: &'a uuid::Uuid,
    pub name: &'a str,
}
