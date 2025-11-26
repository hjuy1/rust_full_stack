use crate::{
    db_access::course::*,
    error::MyError,
    models::course::{CreateCourse, UpdateCourse},
    state::AppState,
};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Json, Response},
};
use std::sync::Arc;

pub async fn post_new_course(
    State(app_state): State<Arc<AppState>>,
    Json(new_course): Json<CreateCourse>,
) -> Result<Response, MyError> {
    println!("post_new_course");
    post_new_course_db(&app_state.db, new_course)
        .await
        .map(|course| Json(course).into_response())
}

pub async fn get_courses_for_teacher(
    State(app_state): State<Arc<AppState>>,
    Path(teacher_id): Path<i32>,
) -> Result<Response, MyError> {
    println!("get_courses_for_teacher");
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| Json(courses).into_response())
}

pub async fn get_course_detail(
    State(app_state): State<Arc<AppState>>,
    Path((teacher_id, course_id)): Path<(i32, i32)>,
) -> Result<Response, MyError> {
    println!("get_course_detail");
    get_course_details_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| Json(course).into_response())
}

pub async fn delete_course(
    State(app_state): State<Arc<AppState>>,
    Path((teacher_id, course_id)): Path<(i32, i32)>,
) -> Result<Response, MyError> {
    println!("delete_course");
    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|resp| Json(resp).into_response())
}

pub async fn update_course_details(
    State(app_state): State<Arc<AppState>>,
    Path((teacher_id, course_id)): Path<(i32, i32)>,
    Json(update_course): Json<UpdateCourse>,
) -> Result<Response, MyError> {
    println!("update_course_details");
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course)
        .await
        .map(|course| Json(course).into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{extract::State, http::StatusCode};
    use sqlx::postgres::PgPoolOptions;
    use std::{env, sync::atomic::AtomicUsize};

    // #[ignore]
    #[tokio::test]
    async fn post_course_test() {
        dotenvy::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: State<Arc<AppState>> = State(Arc::new(AppState {
            health_check_response: "".to_string(),
            visit_count: AtomicUsize::new(0),
            db: db_pool,
        }));

        let course = Json(CreateCourse {
            teacher_id: 1,
            name: "Test course".into(),
            description: Some("This is a course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("English".into()),
            level: Some("Beginner".into()),
        });

        let resp = post_new_course(app_state, course).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_all_courses_success() {
        dotenvy::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: State<Arc<AppState>> = State(Arc::new(AppState {
            health_check_response: "".to_string(),
            visit_count: AtomicUsize::new(0),
            db: db_pool,
        }));

        let teacher_id: Path<i32> = Path(1);
        let resp = get_courses_for_teacher(app_state, teacher_id)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_one_course_success() {
        dotenvy::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: State<Arc<AppState>> = State(Arc::new(AppState {
            health_check_response: "".to_string(),
            visit_count: AtomicUsize::new(0),
            db: db_pool,
        }));

        let params: Path<(i32, i32)> = Path((1, 1));
        let resp = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    // #[should_panic]
    async fn get_one_course_failure() {
        dotenvy::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: State<Arc<AppState>> = State(Arc::new(AppState {
            health_check_response: "".to_string(),
            visit_count: AtomicUsize::new(0),
            db: db_pool,
        }));

        let params: Path<(i32, i32)> = Path((1, 100));
        let resp = get_course_detail(app_state, params).await;
        match resp {
            Ok(_) => println!("Something went wrong"),
            Err(err) => println!("{err}"),
        }
    }

    #[tokio::test]
    // #[ignore]
    async fn delete_course_success() {
        dotenvy::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: State<Arc<AppState>> = State(Arc::new(AppState {
            health_check_response: "".to_string(),
            visit_count: AtomicUsize::new(0),
            db: db_pool,
        }));

        let params: Path<(i32, i32)> = Path((1, 3));
        let resp = delete_course(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn delete_course_failure() {
        dotenvy::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: State<Arc<AppState>> = State(Arc::new(AppState {
            health_check_response: "".to_string(),
            visit_count: AtomicUsize::new(0),
            db: db_pool,
        }));

        let params: Path<(i32, i32)> = Path((1, 301));
        let resp = delete_course(app_state, params).await;
        match resp {
            Ok(_) => println!("Something went wrong"),
            Err(err) => println!("{err}"),
        }
    }
}
