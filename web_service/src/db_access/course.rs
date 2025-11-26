use crate::error::MyError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as(
        r"SELECT * FROM course
        WHERE teacher_id = $1",
    )
    .bind(teacher_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let row = sqlx::query_as(
        r"SELECT * FROM course
        WHERE teacher_id = $1 and id = $2",
    )
    .bind(teacher_id)
    .bind(course_id)
    .fetch_optional(pool)
    .await?;

    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course didn't founded".into()))
    }
}

pub async fn post_new_course_db(
    pool: &PgPool,
    new_course: CreateCourse,
) -> Result<Course, MyError> {
    let row = sqlx::query_as(
        r"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level"      
        ).bind(new_course.teacher_id).bind(new_course.name).bind(new_course.description)
        .bind(new_course.format).bind(new_course.structure).bind(new_course.duration).bind(new_course.price).bind(new_course.language).bind(new_course.level)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn delete_course_db(pool: &PgPool, teacher_id: i32, id: i32) -> Result<String, MyError> {
    let course_row = sqlx::query("DELETE FROM course where teacher_id = $1 and id=$2")
        .bind(teacher_id)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(format!("DeletedI{course_row:?}record"))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse,
) -> Result<Course, MyError> {
    let current_course_row: Course =
        sqlx::query_as("SELECT * FROM course where teacher_id=$1 and id=$2")
            .bind(teacher_id)
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|_err| MyError::NotFound("Course Id not found".into()))?;

    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name
    };
    let description: String = if let Some(description) = update_course.description {
        description
    } else {
        current_course_row.description.unwrap_or_default()
    };
    let format: String = if let Some(format) = update_course.format {
        format
    } else {
        current_course_row.format.unwrap_or_default()
    };
    let structure: String = if let Some(structure) = update_course.structure {
        structure
    } else {
        current_course_row.structure.unwrap_or_default()
    };
    let duration: String = if let Some(duration) = update_course.duration {
        duration
    } else {
        current_course_row.duration.unwrap_or_default()
    };
    let level: String = if let Some(level) = update_course.level {
        level
    } else {
        current_course_row.level.unwrap_or_default()
    };
    let language: String = if let Some(language) = update_course.language {
        language
    } else {
        current_course_row.language.unwrap_or_default()
    };
    let price: i32 = if let Some(price) = update_course.price {
        price
    } else {
        current_course_row.price.unwrap_or_default()
    };
    let course_row = sqlx::query_as(
        "UPDATE course SET name = $1, description = $2, format = $3,
            structure = $4, duration = $5, price = $6, language = $7,
            level = $8 where teacher_id = $9 and id = $10
            RETURNING id, teacher_id, name, time,
            description, format, structure,duration, price, language, level",
    )
    .bind(name)
    .bind(description)
    .bind(format)
    .bind(structure)
    .bind(duration)
    .bind(price)
    .bind(language)
    .bind(level)
    .bind(teacher_id)
    .bind(id)
    .fetch_one(pool)
    .await;
    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course id not found".into()))
    }
}
