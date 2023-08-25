use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse<'a> {
    status: &'a str,
}

pub async fn check() -> (StatusCode, Json<HealthCheckResponse<'static>>) {
    (StatusCode::OK, Json(HealthCheckResponse { status: "ok" }))
}
