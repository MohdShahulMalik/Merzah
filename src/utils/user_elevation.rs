use surrealdb::{engine::remote::ws::Client, RecordId, Surreal};

use crate::{errors::user_elevation::UserElevationError, models::user::{UpdateUser, User}};

pub async fn elevate_user(
    app_admin: RecordId,
    user_being_elevated_id: RecordId,
    elevation_degree: String,
    db: &Surreal<Client>
) -> Result<String, UserElevationError> {

    let admin_check: Option<User> = db.select(app_admin)
        .await
        .map_err(UserElevationError::DatabaseError)?;

    match admin_check {
        Some(admin) => {
            if !admin.is_app_admin() {
                Err(UserElevationError::Unauthorized)?;
            }
        },
        None => Err(UserElevationError::AdminNotFound)?,
    }

    // 3. Fetch the target user
    let check_user_being_elevated: Option<User> = db.select(user_being_elevated_id)
        .await
        .map_err(UserElevationError::DatabaseError)?;

    let mut user_being_elevated = match check_user_being_elevated {
        Some(user) => user,
        None => return Err(UserElevationError::TargetUserNotFound),
    };

    if user_being_elevated.is_mosque_supervisor() {
        Err(UserElevationError::AlreadyElevated("mosque supervisor".to_string()))?
    }

    user_being_elevated.elevate_to(elevation_degree.clone());

    db.update::<Option<User>>(user_being_elevated.id.clone()) // Clone ID so struct isn't partially moved
        .merge::<UpdateUser>(user_being_elevated.into())      // Move the struct
        .await
        .map_err(UserElevationError::DatabaseError)?;
    
    Ok(format!("Elevated the user to {elevation_degree}"))
}
