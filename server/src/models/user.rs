use chrono::prelude::*;
use diesel::prelude::*;
use diesel::SqlType;

use serde::{Deserialize, Serialize};

#[derive(Default, SqlType)]
pub enum UserProvider {
    #[default]
    Google,
}

#[derive(Serialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    id: uuid::Uuid,
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
