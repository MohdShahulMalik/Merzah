# Educational Hub Implementation Plan

> **Project:** Merzah - Islamic Community Platform  
> **Feature:** Educational Hub  
> **Scope:** Phases 1-4 (Core Learning + Gamification + Quizzes + Roadmaps & Frameworks)

---

## Overview

The Educational Hub is Merzah's structured learning platform combining religious (deen) and worldly (dunya) knowledge within Islamic ethical boundaries. It transforms fragmented Islamic content into organized, trackable learning paths.

### Key Decisions

| Aspect | Decision |
|--------|----------|
| Content Storage | External URLs (YouTube, Vimeo, Cloudinary) for media |
| Lesson Content | Single `content` field (text or JSON blob) |
| Analytics | Basic counts (enrollment, completion) - denormalized |
| Deletion | Soft delete with `deleted` boolean field |
| Educator Access | Manual approval by admins via role elevation |
| Content Import | Separate CLI binary (`cargo run --bin import`) |

---

## Content Structure

```
Track → Course → Module → Lesson
```

### Learning Tracks

| Track | Subcategories |
|-------|---------------|
| **Faith & Worship (Deen)** | Aqeedah, Fiqh of 5 Pillars, Quran, Seerah, Akhlaq |
| **Life Skills** | Time Management, Family & Parenting, Mental Resilience, Leadership |
| **Career & Professional** | Study Skills, Career Planning, Tech Skills, Communication |
| **Finance & Wealth** | Halal Finance, Riba Avoidance, Entrepreneurship, Zakah Planning |

---

## Data Model Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           CONTENT HIERARCHY                              │
├─────────────────────────────────────────────────────────────────────────┤
│  Track ──has──> Course ──contains──> Module ──contains──> Lesson       │
│    │                │                      │                   │        │
│    v                v                      v                   v        │
│  4 tracks      Many courses          3-8 modules         5-20 lessons   │
│  (fixed)       per track             per course          per module     │
├─────────────────────────────────────────────────────────────────────────┤
│                         USER PROGRESS                                    │
├─────────────────────────────────────────────────────────────────────────┤
│  User ──enrolled──> Course          User ──completed──> Lesson          │
│         (edge)                              (edge)                       │
├─────────────────────────────────────────────────────────────────────────┤
│                       GAMIFICATION & QUIZZES                             │
├─────────────────────────────────────────────────────────────────────────┤
│  Quiz ──has──> Question ──answered_by──> QuizAttempt                    │
│  User ──has_streak──> UserStreak                                        │
│  User ──earned──> Achievement (edge)                                    │
│  Course ──awards──> Certificate ──earned_by──> User                     │
├─────────────────────────────────────────────────────────────────────────┤
│                      ROADMAPS & FRAMEWORKS                               │
├─────────────────────────────────────────────────────────────────────────┤
│  Roadmap ──includes──> Course (ordered, optional/required)              │
│     │                                                                    │
│     └──> image_url (visual representation)                              │
│                                                                          │
│  Framework ──has──> Milestone ──contains──> Course/Resource             │
│       │                                                                  │
│       └──> image_url (diagram/visualization)                            │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Phase 1: Core Learning Infrastructure

### Goal
Enable basic course discovery, enrollment, and progress tracking.

### 1.1 Database Schemas

#### `schemas/tracks.surql`
```surql
DEFINE TABLE IF NOT EXISTS tracks SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON tracks TYPE string;
DEFINE FIELD IF NOT EXISTS slug ON tracks TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD IF NOT EXISTS description ON tracks TYPE string;
DEFINE FIELD IF NOT EXISTS icon ON tracks TYPE option<string>;
DEFINE FIELD IF NOT EXISTS image_url ON tracks TYPE option<string>;
DEFINE FIELD IF NOT EXISTS sort_order ON tracks TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS created_at ON tracks TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON tracks TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON tracks TYPE bool DEFAULT false;

DEFINE INDEX IF NOT EXISTS track_slug_idx ON tracks FIELDS slug UNIQUE;
```

#### `schemas/courses.surql`
```surql
DEFINE TABLE IF NOT EXISTS courses SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON courses TYPE string;
DEFINE FIELD IF NOT EXISTS slug ON courses TYPE string;
DEFINE FIELD IF NOT EXISTS description ON courses TYPE string;
DEFINE FIELD IF NOT EXISTS short_description ON courses TYPE string;
DEFINE FIELD IF NOT EXISTS track ON courses TYPE record<tracks>;
DEFINE FIELD IF NOT EXISTS educator ON courses TYPE record<users>;
DEFINE FIELD IF NOT EXISTS level ON courses TYPE string 
    ASSERT $value IN ['beginner', 'intermediate', 'advanced'];
DEFINE FIELD IF NOT EXISTS status ON courses TYPE string 
    ASSERT $value IN ['draft', 'review', 'published', 'archived']
    DEFAULT 'draft';
DEFINE FIELD IF NOT EXISTS language ON courses TYPE string DEFAULT 'en';
DEFINE FIELD IF NOT EXISTS thumbnail_url ON courses TYPE option<string>;
DEFINE FIELD IF NOT EXISTS duration_minutes ON courses TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS lesson_count ON courses TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS enrollment_count ON courses TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS created_at ON courses TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON courses TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON courses TYPE bool DEFAULT false;

DEFINE INDEX IF NOT EXISTS course_slug_idx ON courses FIELDS slug UNIQUE;
DEFINE INDEX IF NOT EXISTS course_track_idx ON courses FIELDS track;
DEFINE INDEX IF NOT EXISTS course_status_idx ON courses FIELDS status;
```

#### `schemas/modules.surql`
```surql
DEFINE TABLE IF NOT EXISTS modules SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON modules TYPE string;
DEFINE FIELD IF NOT EXISTS course ON modules TYPE record<courses>;
DEFINE FIELD IF NOT EXISTS description ON modules TYPE option<string>;
DEFINE FIELD IF NOT EXISTS sort_order ON modules TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS created_at ON modules TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON modules TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON modules TYPE bool DEFAULT false;

DEFINE INDEX IF NOT EXISTS module_course_idx ON modules FIELDS course;
```

#### `schemas/lessons.surql`
```surql
DEFINE TABLE IF NOT EXISTS lessons SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON lessons TYPE string;
DEFINE FIELD IF NOT EXISTS module ON lessons TYPE record<modules>;
DEFINE FIELD IF NOT EXISTS content_type ON lessons TYPE string 
    ASSERT $value IN ['text', 'video', 'audio', 'pdf', 'external_link', 'mixed'];
DEFINE FIELD IF NOT EXISTS content ON lessons TYPE string;
DEFINE FIELD IF NOT EXISTS video_url ON lessons TYPE option<string>;
DEFINE FIELD IF NOT EXISTS video_duration_seconds ON lessons TYPE option<int>;
DEFINE FIELD IF NOT EXISTS audio_url ON lessons TYPE option<string>;
DEFINE FIELD IF NOT EXISTS pdf_url ON lessons TYPE option<string>;
DEFINE FIELD IF NOT EXISTS external_url ON lessons TYPE option<string>;
DEFINE FIELD IF NOT EXISTS thumbnail_url ON lessons TYPE option<string>;
DEFINE FIELD IF NOT EXISTS duration_minutes ON lessons TYPE int DEFAULT 5;
DEFINE FIELD IF NOT EXISTS sort_order ON lessons TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS is_preview ON lessons TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS created_at ON lessons TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON lessons TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON lessons TYPE bool DEFAULT false;

DEFINE INDEX IF NOT EXISTS lesson_module_idx ON lessons FIELDS module;
```

#### `schemas/enrolled.surql` (edge table)
```surql
DEFINE TABLE IF NOT EXISTS enrolled SCHEMAFULL TYPE RELATION FROM users TO courses;

DEFINE FIELD IF NOT EXISTS enrolled_at ON enrolled TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS completed_at ON enrolled TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS progress_percent ON enrolled TYPE float DEFAULT 0;
DEFINE FIELD IF NOT EXISTS last_accessed_at ON enrolled TYPE option<datetime>;
```

#### `schemas/completed.surql` (edge table)
```surql
DEFINE TABLE IF NOT EXISTS completed SCHEMAFULL TYPE RELATION FROM users TO lessons;

DEFINE FIELD IF NOT EXISTS completed_at ON completed TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS time_spent_seconds ON completed TYPE option<int>;
```

### 1.2 Models

**File:** `src/models/education.rs`

```rust
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

// ===== CREATE DTOs =====

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
    pub track: RecordId,
    #[garde(skip)]
    pub level: CourseLevel,
    #[garde(skip)]
    pub language: String,
    #[garde(skip)]
    pub thumbnail_url: Option<String>,
}

// Similar CreateModule, CreateLesson, UpdateCourse, etc.
```

### 1.3 Server Functions

**File:** `src/server_functions/education.rs`

#### Public Endpoints (No Auth Required)
| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/education/tracks` | GET | List all tracks with course counts |
| `/education/tracks/{id}/courses` | GET | List courses in a track |
| `/education/courses/{id}` | GET | Get course details with modules/lessons |
| `/education/lessons/{id}` | GET | Get lesson content |
| `/education/search` | GET | Search courses by keyword/level |

#### User Endpoints (Auth Required)
| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/education/enroll` | POST | Enroll in a course |
| `/education/unenroll` | POST | Unenroll from a course |
| `/education/my-courses` | GET | Get user's enrolled courses |
| `/education/complete-lesson` | POST | Mark lesson as complete |
| `/education/progress/{course_id}` | GET | Get progress for a specific course |

#### Educator Endpoints (Educator Role Required)
| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/education/educator/courses` | GET | List educator's courses |
| `/education/educator/courses` | POST | Create new course |
| `/education/educator/courses/{id}` | PATCH | Update course |
| `/education/educator/courses/{id}/publish` | POST | Publish course |
| `/education/educator/modules` | POST | Create module |
| `/education/educator/modules/{id}` | PATCH/DELETE | Update/Delete module |
| `/education/educator/lessons` | POST | Create lesson |
| `/education/educator/lessons/{id}` | PATCH/DELETE | Update/Delete lesson |

### 1.4 Supporting Files

| File | Purpose |
|------|---------|
| `src/utils/education_auth.rs` | `is_course_owner()`, `is_educator_or_admin()` |
| `src/errors/education.rs` | `EducationError` enum |
| `src/services/course_stats.rs` | Update enrollment/lesson counts |

---

## Phase 2: Gamification (Streaks, Achievements, Certificates)

### 2.1 Database Schemas

#### `schemas/user_streaks.surql`
```surql
DEFINE TABLE IF NOT EXISTS user_streaks SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON user_streaks TYPE record<users> UNIQUE;
DEFINE FIELD IF NOT EXISTS current_streak ON user_streaks TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS longest_streak ON user_streaks TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS last_activity_date ON user_streaks TYPE option<string>;
DEFINE FIELD IF NOT EXISTS updated_at ON user_streaks TYPE datetime DEFAULT time::now();
```

#### `schemas/achievements.surql`
```surql
DEFINE TABLE IF NOT EXISTS achievements SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON achievements TYPE string;
DEFINE FIELD IF NOT EXISTS slug ON achievements TYPE string UNIQUE;
DEFINE FIELD IF NOT EXISTS description ON achievements TYPE string;
DEFINE FIELD IF NOT EXISTS icon ON achievements TYPE string;
DEFINE FIELD IF NOT EXISTS category ON achievements TYPE string 
    ASSERT $value IN ['learning', 'streak', 'social', 'milestone'];
DEFINE FIELD IF NOT EXISTS requirement_type ON achievements TYPE string;
DEFINE FIELD IF NOT EXISTS requirement_value ON achievements TYPE int;
DEFINE FIELD IF NOT EXISTS points ON achievements TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS created_at ON achievements TYPE datetime DEFAULT time::now();
```

#### `schemas/earned.surql` (edge)
```surql
DEFINE TABLE IF NOT EXISTS earned SCHEMAFULL TYPE RELATION FROM users TO achievements;

DEFINE FIELD IF NOT EXISTS earned_at ON earned TYPE datetime DEFAULT time::now();
```

#### `schemas/certificates.surql`
```surql
DEFINE TABLE IF NOT EXISTS certificates SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON certificates TYPE record<users>;
DEFINE FIELD IF NOT EXISTS course ON certificates TYPE record<courses>;
DEFINE FIELD IF NOT EXISTS certificate_number ON certificates TYPE string UNIQUE;
DEFINE FIELD IF NOT EXISTS issued_at ON certificates TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS pdf_url ON certificates TYPE option<string>;

DEFINE INDEX IF NOT EXISTS cert_user_course_idx ON certificates FIELDS user, course UNIQUE;
```

### 2.2 Server Functions

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/education/streak` | GET | Get user's current streak |
| `/education/achievements` | GET | Get user's earned achievements |
| `/education/certificates` | GET | Get user's certificates |
| `/education/leaderboard` | GET | Get leaderboard (weekly/monthly/all-time) |

### 2.3 Services

| Service | Purpose |
|---------|---------|
| `src/services/streak.rs` | Increment streak on lesson completion, check streak continuity |
| `src/services/achievement.rs` | Check requirements and award achievements |

---

## Phase 3: Quizzes

### 3.1 Database Schemas

#### `schemas/quizzes.surql`
```surql
DEFINE TABLE IF NOT EXISTS quizzes SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS lesson ON quizzes TYPE record<lessons>;
DEFINE FIELD IF NOT EXISTS title ON quizzes TYPE string;
DEFINE FIELD IF NOT EXISTS passing_score ON quizzes TYPE float DEFAULT 0.7;
DEFINE FIELD IF NOT EXISTS max_attempts ON quizzes TYPE int DEFAULT 3;
DEFINE FIELD IF NOT EXISTS created_at ON quizzes TYPE datetime DEFAULT time::now();
```

#### `schemas/quiz_questions.surql`
```surql
DEFINE TABLE IF NOT EXISTS quiz_questions SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS quiz ON quiz_questions TYPE record<quizzes>;
DEFINE FIELD IF NOT EXISTS question_text ON quiz_questions TYPE string;
DEFINE FIELD IF NOT EXISTS question_type ON quiz_questions TYPE string 
    ASSERT $value IN ['multiple_choice', 'true_false'];
DEFINE FIELD IF NOT EXISTS options ON quiz_questions TYPE option<array<string>>;
DEFINE FIELD IF NOT EXISTS correct_answer ON quiz_questions TYPE string;
DEFINE FIELD IF NOT EXISTS explanation ON quiz_questions TYPE option<string>;
DEFINE FIELD IF NOT EXISTS sort_order ON quiz_questions TYPE int DEFAULT 0;
```

#### `schemas/quiz_attempts.surql`
```surql
DEFINE TABLE IF NOT EXISTS quiz_attempts SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON quiz_attempts TYPE record<users>;
DEFINE FIELD IF NOT EXISTS quiz ON quiz_attempts TYPE record<quizzes>;
DEFINE FIELD IF NOT EXISTS answers ON quiz_attempts TYPE array;
DEFINE FIELD IF NOT EXISTS score ON quiz_attempts TYPE float;
DEFINE FIELD IF NOT EXISTS passed ON quiz_attempts TYPE bool;
DEFINE FIELD IF NOT EXISTS attempted_at ON quiz_attempts TYPE datetime DEFAULT time::now();
```

### 3.2 Server Functions

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/education/quiz/{lesson_id}` | GET | Get quiz for a lesson |
| `/education/quiz/submit` | POST | Submit quiz answers |

---

## Phase 4: Roadmaps & Frameworks

### 4.1 Database Schemas

#### `schemas/roadmaps.surql`
```surql
DEFINE TABLE IF NOT EXISTS roadmaps SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON roadmaps TYPE string;
DEFINE FIELD IF NOT EXISTS slug ON roadmaps TYPE string UNIQUE;
DEFINE FIELD IF NOT EXISTS description ON roadmaps TYPE string;
DEFINE FIELD IF NOT EXISTS image_url ON roadmaps TYPE option<string>;
DEFINE FIELD IF NOT EXISTS track ON roadmaps TYPE option<record<tracks>>;
DEFINE FIELD IF NOT EXISTS difficulty ON roadmaps TYPE string 
    ASSERT $value IN ['beginner', 'intermediate', 'advanced', 'all'];
DEFINE FIELD IF NOT EXISTS estimated_weeks ON roadmaps TYPE int;
DEFINE FIELD IF NOT EXISTS status ON roadmaps TYPE string 
    ASSERT $value IN ['draft', 'published', 'archived'] DEFAULT 'draft';
DEFINE FIELD IF NOT EXISTS created_by ON roadmaps TYPE record<users>;
DEFINE FIELD IF NOT EXISTS created_at ON roadmaps TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON roadmaps TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON roadmaps TYPE bool DEFAULT false;
```

#### `schemas/roadmap_courses.surql` (edge with order)
```surql
DEFINE TABLE IF NOT EXISTS roadmap_courses SCHEMAFULL TYPE RELATION FROM roadmaps TO courses;

DEFINE FIELD IF NOT EXISTS sort_order ON roadmap_courses TYPE int;
DEFINE FIELD IF NOT EXISTS is_required ON roadmap_courses TYPE bool DEFAULT true;
DEFINE FIELD IF NOT EXISTS note ON roadmap_courses TYPE option<string>;
```

#### `schemas/frameworks.surql`
```surql
DEFINE TABLE IF NOT EXISTS frameworks SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON frameworks TYPE string;
DEFINE FIELD IF NOT EXISTS slug ON frameworks TYPE string UNIQUE;
DEFINE FIELD IF NOT EXISTS description ON frameworks TYPE string;
DEFINE FIELD IF NOT EXISTS image_url ON frameworks TYPE option<string>;
DEFINE FIELD IF NOT EXISTS track ON frameworks TYPE option<record<tracks>>;
DEFINE FIELD IF NOT EXISTS status ON frameworks TYPE string DEFAULT 'draft';
DEFINE FIELD IF NOT EXISTS created_by ON frameworks TYPE record<users>;
DEFINE FIELD IF NOT EXISTS created_at ON frameworks TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON frameworks TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON frameworks TYPE bool DEFAULT false;
```

#### `schemas/milestones.surql`
```surql
DEFINE TABLE IF NOT EXISTS milestones SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS framework ON milestones TYPE record<frameworks>;
DEFINE FIELD IF NOT EXISTS title ON milestones TYPE string;
DEFINE FIELD IF NOT EXISTS description ON milestones TYPE option<string>;
DEFINE FIELD IF NOT EXISTS sort_order ON milestones TYPE int;
```

#### `schemas/milestone_courses.surql`
```surql
DEFINE TABLE IF NOT EXISTS milestone_courses SCHEMAFULL TYPE RELATION FROM milestones TO courses;

DEFINE FIELD IF NOT EXISTS is_required ON milestone_courses TYPE bool DEFAULT true;
```

### 4.2 Server Functions

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/education/roadmaps` | GET | List all roadmaps |
| `/education/roadmaps/{id}` | GET | Get roadmap with courses |
| `/education/roadmaps/{id}/start` | POST | Start a roadmap |
| `/education/frameworks` | GET | List all frameworks |
| `/education/frameworks/{id}` | GET | Get framework with milestones |

---

## Content Import Tool

### Purpose
Import existing free courses, roadmaps, and frameworks from JSON files.

### Structure
```
scripts/
├── import.rs              # Main CLI entry point
├── data/
│   ├── tracks.json        # Seed the 4 main tracks
│   ├── courses/           # Individual course JSON files
│   ├── roadmaps/          # Roadmap JSON files
│   └── frameworks/        # Framework JSON files
└── media/
    └── thumbnails/         # Local thumbnail images
```

### Usage
```bash
cargo run --bin import -- --tracks
cargo run --bin import -- --courses
cargo run --bin import -- --roadmaps
cargo run --bin import -- --frameworks
```

---

## Implementation Timeline

| Week | Phase | Tasks |
|------|-------|-------|
| 1 | Phase 1 | Schemas + Models for tracks/courses/modules/lessons |
| 1-2 | Phase 1 | Server functions for public viewing + enrollment |
| 2 | Phase 1 | Progress tracking + educator CRUD |
| 2-3 | Phase 2 | Streaks + achievements |
| 3 | Phase 2 | Certificates |
| 3 | Phase 3 | Quizzes |
| 4 | Phase 4 | Roadmaps + frameworks |
| 4 | Phase 4 | Content import CLI |

---

## Authorization Rules

| Action | Who Can Do It |
|--------|---------------|
| View published courses | Anyone (including unauthenticated) |
| Enroll in course | Any authenticated user |
| Create course | Users with `educator` role |
| Edit/delete course | Course owner or `app_admin` |
| Publish course | Course owner (must have modules + lessons) |
| Mark lesson complete | Enrolled users only |
| Award educator role | `app_admin` or `education_supervisor` |

---

## File Structure Summary

```
src/
├── models/
│   ├── education.rs           # Track, Course, Module, Lesson, DTOs
│   ├── gamification.rs        # Streaks, achievements, certificates
│   ├── quiz.rs                # Quiz, Question, Attempt
│   ├── roadmap.rs             # Roadmap, Framework, Milestone
│   ├── import.rs              # Import DTOs
│   └── mod.rs
│
├── server_functions/
│   ├── education.rs           # Main learning endpoints
│   ├── education_gamification.rs
│   ├── education_quiz.rs
│   ├── education_roadmap.rs
│   └── mod.rs
│
├── services/
│   ├── course_stats.rs        # Update course counts
│   ├── streak.rs               # Streak logic
│   └── achievement.rs         # Achievement checking
│
├── utils/
│   └── education_auth.rs      # is_course_owner, is_educator_or_admin
│
├── errors/
│   ├── education.rs
│   └── mod.rs
│
└── bin/
    └── import.rs               # Content import CLI

schemas/
├── tracks.surql
├── courses.surql
├── modules.surql
├── lessons.surql
├── enrolled.surql
├── completed.surql
├── user_streaks.surql
├── achievements.surql
├── earned.surql
├── certificates.surql
├── quizzes.surql
├── quiz_questions.surql
├── quiz_attempts.surql
├── roadmaps.surql
├── roadmap_courses.surql
├── frameworks.surql
├── milestones.surql
└── milestone_courses.surql
```

---

## Testing

All tests should run with `--features ssr`:

```bash
cargo test --features ssr
```

Test patterns to follow:
- Use `tests/common/mod.rs` for test database setup
- Use structs (not `serde_json`) for request/response
- Use `ApiResponse<T>` wrapper for assertions
- Follow existing patterns in `tests/integration/`

---

## External Content Notes

For importing existing free courses:

1. **Videos**: Use YouTube/Vimeo URLs in `video_url` field
2. **Roadmap Images**: Use external image URLs (e.g., hosted on Cloudinary, Imgur)
3. **PDFs**: Host on cloud storage, use `pdf_url` field
4. **Mixed Content**: Use `content_type: "mixed"` with JSON content blob

Example JSON content for mixed lesson:
```json
{
  "type": "mixed",
  "blocks": [
    {"type": "text", "content": "..."},
    {"type": "video", "url": "https://youtube.com/...", "start_time": 0},
    {"type": "reflection", "prompt": "..."}
  ]
}
```
