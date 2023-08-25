// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct UserGroupsRoleEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct UsersProviderEnum;
}

diesel::table! {
    groups (id) {
        #[max_length = 16]
        id -> Binary,
        name -> Text,
        created_at -> Datetime,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserGroupsRoleEnum;

    user_groups (id) {
        #[max_length = 16]
        id -> Binary,
        #[max_length = 6]
        role -> UserGroupsRoleEnum,
        #[max_length = 16]
        user_id -> Binary,
        #[max_length = 16]
        group_id -> Binary,
        created_at -> Datetime,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UsersProviderEnum;

    users (id) {
        #[max_length = 16]
        id -> Binary,
        name -> Text,
        #[max_length = 256]
        email -> Varchar,
        #[max_length = 6]
        provider -> UsersProviderEnum,
        #[max_length = 256]
        provider_id -> Varchar,
        created_at -> Datetime,
    }
}

diesel::allow_tables_to_appear_in_same_query!(groups, user_groups, users,);
