use crate::models::Provider;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub provider: Provider,
    pub provider_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct CreateUser<'a> {
    name: &'a str,
    email: &'a str,
    provider: &'a Provider,
    provider_id: &'a str,
}
