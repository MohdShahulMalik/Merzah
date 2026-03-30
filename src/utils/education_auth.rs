use surrealdb::{RecordId, Surreal, engine::remote::ws::Client};

use crate::errors::education::EducationError;
use crate::models::education::Course;
use crate::models::user::User;

pub async fn is_course_owner(
    user: &User,
    course_id: &RecordId,
    db: &Surreal<Client>,
) -> Result<(), EducationError> {
    if user.role == "app_admin" || user.role == "education_supervisor" {
        return Ok(());
    }

    let course: Option<Course> = db.select(course_id.clone()).await?;

    match course {
        Some(course) if !course.deleted => {
            if course.educator == user.id {
                Ok(())
            } else {
                Err(EducationError::Unauthorized)
            }
        }
        _ => Err(EducationError::NotFound),
    }
}

pub fn is_educator_or_admin(user: &User) -> Result<(), EducationError> {
    if user.role == "educator" || user.role == "app_admin" || user.role == "education_supervisor" {
        Ok(())
    } else {
        Err(EducationError::Unauthorized)
    }
}
