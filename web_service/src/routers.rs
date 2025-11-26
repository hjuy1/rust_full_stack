use crate::{
    handlers::{course::*, general::health_check_handler},
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn general_route() -> Router<Arc<AppState>> {
    Router::new().route("/health", get(health_check_handler))
}

pub fn course_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(post_new_course))
        .route("/{teacher_id}", get(get_courses_for_teacher))
        .route(
            "/{teacher_id}/{course_id}",
            get(get_course_detail)
                .delete(delete_course)
                .put(update_course_details),
        )
}
