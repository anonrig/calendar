use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use std::env;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub struct DatabaseConnection(
    pub bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

pub async fn get_connection_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    Pool::builder().build(config).await.unwrap()
}

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    Pool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);
        let connection = pool
            .get_owned()
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Self(connection))
    }
}
