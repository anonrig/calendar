// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "providers"))]
    pub struct Providers;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "roles"))]
    pub struct Roles;
}

diesel::table! {
    groups (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Roles;

    user_groups (id) {
        id -> Uuid,
        role -> Roles,
        user_id -> Uuid,
        group_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Providers;

    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        provider -> Providers,
        provider_id -> Text,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(user_groups -> groups (group_id));
diesel::joinable!(user_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    user_groups,
    users,
);
