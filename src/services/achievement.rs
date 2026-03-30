use std::collections::HashSet;

use serde::Deserialize;
use surrealdb::{RecordId, Surreal, engine::remote::ws::Client};

use crate::errors::education::EducationError;
use crate::models::gamification::Achievement;

#[derive(Debug, Deserialize)]
struct CountResult {
    pub count: i64,
}

#[derive(Debug, Deserialize)]
struct EarnedRow {
    pub out: RecordId,
}

#[derive(Debug, Deserialize)]
struct StreakRow {
    pub current_streak: i32,
}

pub async fn check_and_award_achievements(
    user_id: &RecordId,
    db: &Surreal<Client>,
) -> Result<Vec<Achievement>, EducationError> {
    let achievements: Vec<Achievement> = db.select("achievements").await?;

    let mut earned_response = db
        .query("SELECT out FROM earned WHERE in = $user_id")
        .bind(("user_id", user_id.clone()))
        .await?;
    let earned_rows: Vec<EarnedRow> = earned_response.take(0)?;
    let earned_set: HashSet<RecordId> = earned_rows.into_iter().map(|row| row.out).collect();

    let completed_lessons = {
        let mut response = db
            .query("SELECT count() AS count FROM completed WHERE in = $user_id")
            .bind(("user_id", user_id.clone()))
            .await?;
        let rows: Vec<CountResult> = response.take(0)?;
        rows.first().map(|c| c.count).unwrap_or(0)
    };

    let completed_courses = {
        let mut response = db
            .query("SELECT count() AS count FROM enrolled WHERE in = $user_id AND completed_at != NONE")
            .bind(("user_id", user_id.clone()))
            .await?;
        let rows: Vec<CountResult> = response.take(0)?;
        rows.first().map(|c| c.count).unwrap_or(0)
    };

    let enrollment_count = {
        let mut response = db
            .query("SELECT count() AS count FROM enrolled WHERE in = $user_id")
            .bind(("user_id", user_id.clone()))
            .await?;
        let rows: Vec<CountResult> = response.take(0)?;
        rows.first().map(|c| c.count).unwrap_or(0)
    };

    let current_streak = {
        let mut response = db
            .query("SELECT current_streak FROM user_streaks WHERE user = $user_id LIMIT 1")
            .bind(("user_id", user_id.clone()))
            .await?;
        let row: Option<StreakRow> = response.take(0)?;
        row.map(|row| row.current_streak).unwrap_or(0)
    };

    let mut newly_earned = Vec::new();
    for achievement in achievements {
        if earned_set.contains(&achievement.id) {
            continue;
        }

        let meets_requirement = match achievement.requirement_type.as_str() {
            "lessons_completed" => completed_lessons >= achievement.requirement_value as i64,
            "courses_completed" => completed_courses >= achievement.requirement_value as i64,
            "enrollments" => enrollment_count >= achievement.requirement_value as i64,
            "streak_days" => current_streak >= achievement.requirement_value,
            _ => false,
        };

        if meets_requirement {
            db.query("RELATE $user_id -> earned -> $achievement_id SET earned_at = time::now()")
                .bind(("user_id", user_id.clone()))
                .bind(("achievement_id", achievement.id.clone()))
                .await?;
            newly_earned.push(achievement);
        }
    }

    Ok(newly_earned)
}
