use chrono::prelude::*;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum UserProvider {
    Google,
}

#[derive(Serialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    id: String,
    name: String,
    email: String,
    provider: UserProvider,
    provider_id: String,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct CreateUser {
    name: String,
    email: String,
    provider: UserProvider,
    provider_id: String,
}
