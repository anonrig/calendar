use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

pub mod group;
pub mod user;
pub mod user_group;

#[derive(Debug, PartialEq, DbEnum, Clone, Deserialize, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::Roles"]
pub enum Role {
    User,
    Owner,
}

#[derive(Debug, PartialEq, DbEnum, Clone, Deserialize, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::Providers"]
pub enum Provider {
    Google,
}
