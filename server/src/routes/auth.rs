use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

pub async fn login(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User { id: 1337, username: payload.username };

    (StatusCode::OK, Json(user))
}

pub async fn redirect() -> (StatusCode, Json<User>) {
    let user = User { id: 1337, username: "hello".to_string() };

    (StatusCode::OK, Json(user))
}
