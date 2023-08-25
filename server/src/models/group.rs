use chrono::prelude::*;
use diesel::prelude::*;
use diesel::SqlType;

use serde::{Deserialize, Serialize};

#[derive(Default, SqlType)]
pub enum GroupMemberRole {
    #[default]
    User,
    Owner,
}

#[derive(Serialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = crate::schema::groups)]
pub struct Group {
    id: uuid::Uuid,
    name: String,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::groups)]
pub struct CreateGroup {
    name: String,
}

#[derive(Serialize, Selectable, Queryable)]
#[diesel(table_name = crate::schema::user_groups)]
pub struct UserGroup {
    id: uuid::Uuid,
    role: GroupMemberRole,
    user_id: uuid::Uuid,
    group_id: uuid::Uuid,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Insertable)]
#[diesel(table_name = crate::schema::user_groups)]
pub struct CreateUserGroup {
    role: GroupMemberRole,
    user_id: uuid::Uuid,
    group_id: uuid::Uuid,
}
