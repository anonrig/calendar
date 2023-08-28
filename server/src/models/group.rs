use chrono::prelude::*;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum GroupMemberRole {
    User,
    Owner,
}

#[derive(Serialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = crate::schema::groups)]
pub struct Group {
    id: String,
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
    id: String,
    role: GroupMemberRole,
    user_id: String,
    group_id: String,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Insertable)]
#[diesel(table_name = crate::schema::user_groups)]
pub struct CreateUserGroup {
    role: GroupMemberRole,
    user_id: String,
    group_id: String,
}
