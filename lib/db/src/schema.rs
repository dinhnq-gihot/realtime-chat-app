// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "message_types"))]
    pub struct MessageTypes;
}

diesel::table! {
    files (message_id) {
        message_id -> Uuid,
        #[max_length = 255]
        filename -> Varchar,
        #[max_length = 255]
        file_path -> Varchar,
    }
}

diesel::table! {
    forwarded_messages (id) {
        id -> Uuid,
        original_message_id -> Nullable<Uuid>,
        new_message_id -> Nullable<Uuid>,
        forwarded_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    groups (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MessageTypes;

    messages (id) {
        id -> Uuid,
        user_id -> Uuid,
        group_id -> Uuid,
        content -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<MessageTypes>,
        created_at -> Timestamptz,
        edited_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        avatar -> Nullable<Varchar>,
        is_online -> Nullable<Bool>,
    }
}

diesel::table! {
    users_groups (group_id, user_id) {
        group_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::joinable!(files -> messages (message_id));
diesel::joinable!(messages -> groups (group_id));
diesel::joinable!(messages -> users (user_id));
diesel::joinable!(users_groups -> groups (group_id));
diesel::joinable!(users_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    files,
    forwarded_messages,
    groups,
    messages,
    users,
    users_groups,
);
