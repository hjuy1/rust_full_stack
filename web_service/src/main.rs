use chrono::NaiveDateTime;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Debug, sqlx::FromRow)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
    let courses_list: Vec<Course> =
        sqlx::query_as("select id, teacher_id, name, time from course where teacher_id = $1")
            .bind(1)
            .fetch_all(&db_pool)
            .await?;

    println!("Courses = {courses_list:?}");

    Ok(())
}
