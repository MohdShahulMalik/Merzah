use serde::{Deserialize, Serialize};

use crate::models::education::{CourseLevel, LessonContentType};
use crate::models::roadmap::RoadmapDifficulty;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportTrack {
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon: Option<String>,
    pub image_url: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportLesson {
    pub title: String,
    pub content_type: LessonContentType,
    pub content: String,
    pub video_url: Option<String>,
    pub video_duration_seconds: Option<i32>,
    pub audio_url: Option<String>,
    pub pdf_url: Option<String>,
    pub external_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_minutes: Option<i32>,
    pub sort_order: Option<i32>,
    pub is_preview: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportModule {
    pub title: String,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub lessons: Vec<ImportLesson>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportCourse {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub short_description: String,
    pub track_slug: String,
    pub educator_id: String,
    pub level: CourseLevel,
    pub language: Option<String>,
    pub thumbnail_url: Option<String>,
    pub modules: Vec<ImportModule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportRoadmapCourse {
    pub course_slug: String,
    pub sort_order: i32,
    pub is_required: Option<bool>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportRoadmap {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub image_url: Option<String>,
    pub track_slug: Option<String>,
    pub difficulty: RoadmapDifficulty,
    pub estimated_weeks: i32,
    pub status: Option<String>,
    pub created_by: String,
    pub courses: Vec<ImportRoadmapCourse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportMilestoneCourse {
    pub course_slug: String,
    pub is_required: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportMilestone {
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub courses: Vec<ImportMilestoneCourse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportFramework {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub image_url: Option<String>,
    pub track_slug: Option<String>,
    pub status: Option<String>,
    pub created_by: String,
    pub milestones: Vec<ImportMilestone>,
}
