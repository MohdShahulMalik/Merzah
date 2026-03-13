use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AchievementCategory {
    Learning,
    Streak,
    Social,
    Milestone,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct UserStreak {
    pub id: RecordId,
    pub user: RecordId,
    pub current_streak: i32,
    pub longest_streak: i32,
    pub last_activity_date: Option<String>,
    pub updated_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserStreakOnClient {
    pub current_streak: i32,
    pub longest_streak: i32,
    pub last_activity_date: Option<String>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Achievement {
    pub id: RecordId,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon: String,
    pub category: AchievementCategory,
    pub requirement_type: String,
    pub requirement_value: i32,
    pub points: i32,
    pub created_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AchievementOnClient {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon: String,
    pub category: AchievementCategory,
    pub points: i32,
    pub earned_at: Option<DateTime<FixedOffset>>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    pub id: RecordId,
    pub user: RecordId,
    pub course: RecordId,
    pub certificate_number: String,
    pub issued_at: Datetime,
    pub pdf_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CertificateOnClient {
    pub id: String,
    pub course_id: String,
    pub certificate_number: String,
    pub issued_at: DateTime<FixedOffset>,
    pub pdf_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaderboardEntry {
    pub user_id: String,
    pub display_name: String,
    pub current_streak: i32,
    pub longest_streak: i32,
}
