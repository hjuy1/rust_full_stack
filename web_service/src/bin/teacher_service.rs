#![deny(clippy::unwrap_used)]
use axum::Router;
use error::MyError;
use routers::{course_routes, general_route};
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use std::{
    env,
    sync::{Arc, atomic::AtomicUsize},
};
use tower_http::services::{ServeDir, ServeFile};

#[path = "../db_access/mod.rs"]
mod db_access;
#[path = "../error.rs"]
pub mod error;
#[path = "../handlers/mod.rs"]
pub mod handlers;
#[path = "../models/mod.rs"]
pub mod models;
#[path = "../routers.rs"]
pub mod routers;
#[path = "../state.rs"]
pub mod state;

#[tokio::main]
async fn main() -> Result<(), MyError> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");

    let db_pool = PgPoolOptions::new().connect(&database_url).await?;
    let state = Arc::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: AtomicUsize::new(0),
        db: db_pool,
    });

    let app = Router::new()
        .merge(general_route())
        .nest("/course", course_routes())
        .nest_service(
            "/static",
            ServeDir::new("static").not_found_service(ServeFile::new("static/404.html")),
        )
        .nest_service(
            "/assert",
            ServeDir::new("assert").not_found_service(ServeFile::new("static/404.html")),
        )
        .fallback_service(ServeFile::new("static/404.html"))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    Ok(axum::serve(listener, app).await?)
}
