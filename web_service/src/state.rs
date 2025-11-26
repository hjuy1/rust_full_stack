// use crate::modelds::Course;
use sqlx::PgPool;
use std::sync::atomic::AtomicUsize;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: AtomicUsize,
    pub db: PgPool,
    // pub courses: Mutex<Vec<Course>>,
}
