use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RoadmapDifficulty {
    Beginner,
    Intermediate,
    Advanced,
    All,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RoadmapStatus {
    Draft,
    Published,
    Archived,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Roadmap {
    pub id: RecordId,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub image_url: Option<String>,
    pub track: Option<RecordId>,
    pub difficulty: RoadmapDifficulty,
    pub estimated_weeks: i32,
    pub status: RoadmapStatus,
    pub created_by: RecordId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub deleted: bool,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoadmapCourseEdge {
    #[serde(rename = "in")]
    pub in_: RecordId,
    pub out: RecordId,
    pub sort_order: i32,
    pub is_required: bool,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoadmapOnClient {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub image_url: Option<String>,
    pub difficulty: RoadmapDifficulty,
    pub estimated_weeks: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoadmapCourseOnClient {
    pub course_id: String,
    pub title: String,
    pub short_description: String,
    pub thumbnail_url: Option<String>,
    pub sort_order: i32,
    pub is_required: bool,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoadmapDetail {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub image_url: Option<String>,
    pub difficulty: RoadmapDifficulty,
    pub estimated_weeks: i32,
    pub courses: Vec<RoadmapCourseOnClient>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Framework {
    pub id: RecordId,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub image_url: Option<String>,
    pub track: Option<RecordId>,
    pub status: String,
    pub created_by: RecordId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub deleted: bool,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Milestone {
    pub id: RecordId,
    pub framework: RecordId,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MilestoneCourseEdge {
    #[serde(rename = "in")]
    pub in_: RecordId,
    pub out: RecordId,
    pub is_required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MilestoneOnClient {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub courses: Vec<RoadmapCourseOnClient>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrameworkOnClient {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrameworkDetail {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub image_url: Option<String>,
    pub milestones: Vec<MilestoneOnClient>,
}
