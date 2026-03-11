use chrono::{DateTime, FixedOffset};
use garde::Validate;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

// ===== TRACKS =====

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TrackSlug {
    FaithWorship,
    LifeSkills,
    CareerProfessional,
    FinanceWealth,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    pub id: RecordId,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon: Option<String>,
    pub image_url: Option<String>,
    pub sort_order: i32,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackOnClient {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon: Option<String>,
    pub image_url: Option<String>,
    pub course_count: usize,
}

// ===== COURSES =====

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CourseLevel {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CourseStatus {
    Draft,
    Review,
    Published,
    Archived,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: RecordId,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub short_description: String,
    pub track: RecordId,
    pub educator: RecordId,
    pub level: CourseLevel,
    pub status: CourseStatus,
    pub language: String,
    pub thumbnail_url: Option<String>,
    pub duration_minutes: i32,
    pub lesson_count: i32,
    pub enrollment_count: i32,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub deleted: bool,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CourseRecord {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub short_description: String,
    pub track: RecordId,
    pub educator: RecordId,
    pub level: CourseLevel,
    pub status: CourseStatus,
    pub language: String,
    pub thumbnail_url: Option<String>,
    pub duration_minutes: i32,
    pub lesson_count: i32,
    pub enrollment_count: i32,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub deleted: bool,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatedCourseRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<RecordId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<CourseLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CourseStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<i32>,
    pub updated_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CourseOnClient {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub short_description: String,
    pub level: CourseLevel,
    pub thumbnail_url: Option<String>,
    pub duration_minutes: i32,
    pub lesson_count: i32,
    pub enrollment_count: i32,
    pub educator_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CourseDetail {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub short_description: String,
    pub level: CourseLevel,
    pub thumbnail_url: Option<String>,
    pub duration_minutes: i32,
    pub lesson_count: i32,
    pub enrollment_count: i32,
    pub educator: EducatorInfo,
    pub modules: Vec<ModuleWithLessons>,
    pub is_enrolled: bool,
    pub progress_percent: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EducatorInfo {
    pub id: String,
    pub display_name: String,
}

// ===== MODULES =====

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub id: RecordId,
    pub title: String,
    pub course: RecordId,
    pub description: Option<String>,
    pub sort_order: i32,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub deleted: bool,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleRecord {
    pub title: String,
    pub course: RecordId,
    pub description: Option<String>,
    pub sort_order: i32,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub deleted: bool,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatedModuleRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
    pub updated_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleOnClient {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub lesson_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleWithLessons {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub lessons: Vec<LessonOnClient>,
}

// ===== LESSONS =====

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LessonContentType {
    Text,
    Video,
    Audio,
    Pdf,
    ExternalLink,
    Mixed,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub id: RecordId,
    pub title: String,
    pub module: RecordId,
    pub content_type: LessonContentType,
    pub content: String,
    pub video_url: Option<String>,
    pub video_duration_seconds: Option<i32>,
    pub audio_url: Option<String>,
    pub pdf_url: Option<String>,
    pub external_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_minutes: i32,
    pub sort_order: i32,
    pub is_preview: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub deleted: bool,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LessonRecord {
    pub title: String,
    pub module: RecordId,
    pub content_type: LessonContentType,
    pub content: String,
    pub video_url: Option<String>,
    pub video_duration_seconds: Option<i32>,
    pub audio_url: Option<String>,
    pub pdf_url: Option<String>,
    pub external_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_minutes: i32,
    pub sort_order: i32,
    pub is_preview: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub deleted: bool,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatedLessonRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<LessonContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
    pub updated_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LessonOnClient {
    pub id: String,
    pub title: String,
    pub content_type: LessonContentType,
    pub thumbnail_url: Option<String>,
    pub duration_minutes: i32,
    pub sort_order: i32,
    pub is_preview: bool,
    pub is_completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LessonDetail {
    pub id: String,
    pub title: String,
    pub content_type: LessonContentType,
    pub content: String,
    pub video_url: Option<String>,
    pub video_duration_seconds: Option<i32>,
    pub audio_url: Option<String>,
    pub pdf_url: Option<String>,
    pub external_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_minutes: i32,
    pub module_id: String,
    pub module_title: String,
    pub course_id: String,
    pub course_title: String,
    pub is_completed: bool,
    pub next_lesson_id: Option<String>,
    pub prev_lesson_id: Option<String>,
}

// ===== ENROLLMENT & PROGRESS =====

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnrollmentProgress {
    pub course_id: String,
    pub course_title: String,
    pub thumbnail_url: Option<String>,
    pub enrolled_at: DateTime<FixedOffset>,
    pub progress_percent: f32,
    pub completed_lessons: i32,
    pub total_lessons: i32,
    pub last_accessed_at: Option<DateTime<FixedOffset>>,
}

// ===== CREATE/UPDATE DTOs =====

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct CreateCourse {
    #[garde(length(min = 3, max = 200))]
    pub title: String,
    #[garde(length(min = 3, max = 100))]
    pub slug: String,
    #[garde(length(min = 10, max = 5000))]
    pub description: String,
    #[garde(length(min = 10, max = 200))]
    pub short_description: String,
    #[garde(skip)]
    pub track: String,
    #[garde(skip)]
    pub level: CourseLevel,
    #[garde(skip)]
    pub language: String,
    #[garde(skip)]
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct UpdateCourse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(inner(length(min = 3, max = 200)))]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(inner(length(min = 3, max = 100)))]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(inner(length(min = 10, max = 5000)))]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(inner(length(min = 10, max = 200)))]
    pub short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub track: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub level: Option<CourseLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub status: Option<CourseStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub duration_minutes: Option<i32>,
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct CreateModule {
    #[garde(length(min = 3, max = 200))]
    pub title: String,
    #[garde(skip)]
    pub course: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(inner(length(min = 3, max = 500)))]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub sort_order: Option<i32>,
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct UpdateModule {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(inner(length(min = 3, max = 200)))]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(inner(length(min = 3, max = 500)))]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub sort_order: Option<i32>,
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct CreateLesson {
    #[garde(length(min = 3, max = 200))]
    pub title: String,
    #[garde(skip)]
    pub module: String,
    #[garde(skip)]
    pub content_type: LessonContentType,
    #[garde(length(min = 1, max = 50000))]
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub video_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub video_duration_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub audio_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub pdf_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub external_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub duration_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub sort_order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub is_preview: Option<bool>,
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct UpdateLesson {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(inner(length(min = 3, max = 200)))]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub content_type: Option<LessonContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(inner(length(min = 1, max = 50000)))]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub video_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub video_duration_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub audio_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub pdf_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub external_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub duration_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub sort_order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub is_preview: Option<bool>,
}
