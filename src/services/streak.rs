use chrono::{Datelike, NaiveDate, Utc};
use surrealdb::{RecordId, Surreal, engine::remote::ws::Client};

use crate::errors::education::EducationError;
use crate::models::gamification::UserStreak;

fn parse_date(date_str: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
}

fn today_string() -> String {
    let today = Utc::now().date_naive();
    format!(
        "{:04}-{:02}-{:02}",
        today.year(),
        today.month(),
        today.day()
    )
}

pub async fn bump_streak(
    user_id: &RecordId,
    db: &Surreal<Client>,
) -> Result<UserStreak, EducationError> {
    let today = today_string();

    let existing: Option<UserStreak> = db
        .query("SELECT * FROM user_streaks WHERE user = $user_id LIMIT 1")
        .bind(("user_id", user_id.clone()))
        .await?
        .take(0)?;

    let updated = match existing {
        Some(mut streak) => {
            let last_date = streak.last_activity_date.as_deref().and_then(parse_date);

            if let Some(last_date) = last_date {
                let today_date = parse_date(&today).unwrap_or(last_date);
                let diff = (today_date - last_date).num_days();
                if diff == 0 {
                    return Ok(streak);
                }

                if diff == 1 {
                    streak.current_streak += 1;
                } else {
                    streak.current_streak = 1;
                }
            } else {
                streak.current_streak = 1;
            }

            if streak.current_streak > streak.longest_streak {
                streak.longest_streak = streak.current_streak;
            }

            streak.last_activity_date = Some(today.clone());
            streak.updated_at = Utc::now().into();

            let updated: Option<UserStreak> = db.update(streak.id.clone()).merge(streak).await?;
            updated.ok_or(EducationError::NotFound)?
        }
        None => {
            let create_query = r#"
                CREATE user_streaks CONTENT {
                    user: $user_id,
                    current_streak: 1,
                    longest_streak: 1,
                    last_activity_date: $today,
                    updated_at: time::now()
                }
            "#;
            let created: Option<UserStreak> = db
                .query(create_query)
                .bind(("user_id", user_id.clone()))
                .bind(("today", today))
                .await?
                .take(0)?;
            created.ok_or(EducationError::NotFound)?
        }
    };

    Ok(updated)
}
