use crate::models::{group::Group, user::User, Role};
use chrono::{DateTime, Utc};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::user_groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(primary_key(id))]
pub struct UserGroup {
    pub id: Uuid,
    pub role: Role,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user_groups)]
pub struct CreateUserGroup<'a> {
    role: &'a Role,
    user_id: &'a Uuid,
    group_id: &'a Uuid,
}
