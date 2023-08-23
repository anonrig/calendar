mod error;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use google_calendar::Client;
use std::{env, sync::Arc};

pub struct GoogleCalendar;

impl GoogleCalendar {
    pub fn get_client(token: &str, refresh_token: &str) -> Client {
        Client::new(
            env::var("GOOGLE_CALENDAR_CLIENT_ID").expect("GOOGLE_CALENDAR_CLIENT_ID"),
            env::var("GOOGLE_CALENDAR_CLIENT_SECRET").expect("GOOGLE_CALENDAR_CLIENT_SECRET"),
            env::var("GOOGLE_CALENDAR_REDIRECT_URI").expect("GOOGLE_CALENDAR_REDIRECT_URI"),
            token,
            refresh_token,
        )
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let google_calendar = Arc::new(GoogleCalendar);

    let app = Router::new()
        .route("/auth/redirect", get(routes::auth::redirect))
        .route("/auth/login", post(routes::auth::login))
        .with_state(google_calendar);

    tracing::info!("Running server on 0.0.0.0:4000");

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
