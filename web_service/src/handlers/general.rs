use crate::state::AppState;
use axum::{
    extract::State,
    response::{IntoResponse, Json, Response},
};
use std::sync::{Arc, atomic::Ordering};

pub async fn health_check_handler(State(app_state): State<Arc<AppState>>) -> Response {
    println!("incoming for health check");
    let health_check_response = &app_state.health_check_response;
    let visit_count = &app_state.visit_count;
    let response = format!(
        "{health_check_response} {} times",
        visit_count.load(Ordering::Relaxed)
    );
    visit_count.fetch_add(1, Ordering::Relaxed);
    Json(response).into_response()
}
