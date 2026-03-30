use serde::Deserialize;
use surrealdb::{RecordId, Surreal, engine::remote::ws::Client};

use crate::errors::education::EducationError;

#[derive(Debug, Deserialize)]
struct CountResult {
    pub count: i64,
}

pub async fn update_course_lesson_count(
    course_id: &RecordId,
    db: &Surreal<Client>,
) -> Result<(), EducationError> {
    let mut response = db
        .query(
            "SELECT count() AS count FROM lessons
            WHERE module IN (SELECT VALUE id FROM modules WHERE course = $course_id AND deleted = false)
            AND deleted = false",
        )
        .bind(("course_id", course_id.clone()))
        .await?;
    let rows: Vec<CountResult> = response.take(0)?;
    let lesson_count = rows.first().map(|row| row.count).unwrap_or(0);

    db.query("UPDATE ONLY $course_id SET lesson_count = $lesson_count, updated_at = time::now()")
        .bind(("course_id", course_id.clone()))
        .bind(("lesson_count", lesson_count))
        .await?;

    Ok(())
}

pub async fn update_course_enrollment_count(
    course_id: &RecordId,
    db: &Surreal<Client>,
) -> Result<(), EducationError> {
    let mut response = db
        .query("SELECT count() AS count FROM enrolled WHERE out = $course_id")
        .bind(("course_id", course_id.clone()))
        .await?;
    let rows: Vec<CountResult> = response.take(0)?;
    let enrollment_count = rows.first().map(|row| row.count).unwrap_or(0);

    db.query(
        "UPDATE ONLY $course_id SET enrollment_count = $enrollment_count, updated_at = time::now()",
    )
    .bind(("course_id", course_id.clone()))
    .bind(("enrollment_count", enrollment_count))
    .await?;

    Ok(())
}
