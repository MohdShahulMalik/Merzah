#[cfg(feature = "ssr")]
use std::path::{Path, PathBuf};
#[cfg(feature = "ssr")]
use std::{env, fs};

#[cfg(feature = "ssr")]
use chrono::Utc;
#[cfg(feature = "ssr")]
use serde::Deserialize;
#[cfg(feature = "ssr")]
use serde::de::DeserializeOwned;
#[cfg(feature = "ssr")]
use surrealdb::Datetime;
#[cfg(feature = "ssr")]
use surrealdb::RecordId;

#[cfg(feature = "ssr")]
use merzah::database::connection::init_db;
#[cfg(feature = "ssr")]
use merzah::models::education::{CourseRecord, CourseStatus, LessonRecord, ModuleRecord};
#[cfg(feature = "ssr")]
use merzah::models::import::{ImportCourse, ImportFramework, ImportRoadmap, ImportTrack};
#[cfg(feature = "ssr")]
use merzah::services::course_stats::update_course_lesson_count;

#[cfg(feature = "ssr")]
fn read_json_file<T: DeserializeOwned>(path: &PathBuf) -> anyhow::Result<T> {
    let raw = fs::read_to_string(path)?;
    let data = serde_json::from_str(&raw)?;
    Ok(data)
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct IdRow {
    pub id: RecordId,
}

#[cfg(feature = "ssr")]
async fn import_tracks(base: &Path) -> anyhow::Result<()> {
    let mut path: PathBuf = base.to_path_buf();
    path.push("tracks.json");
    if !path.exists() {
        anyhow::bail!("tracks.json not found at {}", path.display());
    }
    let tracks: Vec<ImportTrack> = read_json_file(&path)?;

    let db = init_db().await;
    db.query("INSERT INTO tracks $tracks")
        .bind(("tracks", tracks))
        .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
async fn import_courses(base: &Path) -> anyhow::Result<()> {
    let mut dir = base.to_path_buf();
    dir.push("courses");
    if !dir.exists() {
        anyhow::bail!("courses directory not found at {}", dir.display());
    }

    let db = init_db().await;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let course: ImportCourse = read_json_file(&path)?;

        let mut track_response = db
            .query("SELECT id FROM tracks WHERE slug = $slug LIMIT 1")
            .bind(("slug", course.track_slug.clone()))
            .await?;
        let track_row: Option<IdRow> = track_response.take(0)?;
        let track_id = track_row
            .map(|row| row.id)
            .ok_or_else(|| anyhow::anyhow!("Track not found for slug {}", course.track_slug))?;

        let educator_id: RecordId = course.educator_id.parse()?;

        let now: Datetime = Utc::now().into();
        let record = CourseRecord {
            title: course.title,
            slug: course.slug,
            description: course.description,
            short_description: course.short_description,
            track: track_id,
            educator: educator_id,
            level: course.level,
            status: CourseStatus::Published,
            language: course.language.unwrap_or_else(|| "en".to_string()),
            thumbnail_url: course.thumbnail_url,
            duration_minutes: 0,
            lesson_count: 0,
            enrollment_count: 0,
            created_at: now.clone(),
            updated_at: now,
            deleted: false,
        };

        let mut course_response = db
            .query("CREATE courses CONTENT $course")
            .bind(("course", record))
            .await?;
        let created_course: Option<IdRow> = course_response.take(0)?;
        let course_id = created_course
            .map(|row| row.id)
            .ok_or_else(|| anyhow::anyhow!("Failed to create course"))?;

        for module in course.modules {
            let now: Datetime = Utc::now().into();
            let module_record = ModuleRecord {
                title: module.title,
                course: course_id.clone(),
                description: module.description,
                sort_order: module.sort_order.unwrap_or(0),
                created_at: now.clone(),
                updated_at: now,
                deleted: false,
            };
            let mut module_response = db
                .query("CREATE modules CONTENT $module")
                .bind(("module", module_record))
                .await?;
            let created_module: Option<IdRow> = module_response.take(0)?;
            let module_id = created_module
                .map(|row| row.id)
                .ok_or_else(|| anyhow::anyhow!("Failed to create module"))?;

            for lesson in module.lessons {
                let now: Datetime = Utc::now().into();
                let lesson_record = LessonRecord {
                    title: lesson.title,
                    module: module_id.clone(),
                    content_type: lesson.content_type,
                    content: lesson.content,
                    video_url: lesson.video_url,
                    video_duration_seconds: lesson.video_duration_seconds,
                    audio_url: lesson.audio_url,
                    pdf_url: lesson.pdf_url,
                    external_url: lesson.external_url,
                    thumbnail_url: lesson.thumbnail_url,
                    duration_minutes: lesson.duration_minutes.unwrap_or(5),
                    sort_order: lesson.sort_order.unwrap_or(0),
                    is_preview: lesson.is_preview.unwrap_or(false),
                    created_at: now.clone(),
                    updated_at: now,
                    deleted: false,
                };

                db.query("CREATE lessons CONTENT $lesson")
                    .bind(("lesson", lesson_record))
                    .await?;
            }
        }

        let _ = update_course_lesson_count(&course_id, &db).await;
    }

    Ok(())
}

#[cfg(feature = "ssr")]
async fn import_roadmaps(base: &Path) -> anyhow::Result<()> {
    let mut dir = base.to_path_buf();
    dir.push("roadmaps");
    if !dir.exists() {
        anyhow::bail!("roadmaps directory not found at {}", dir.display());
    }

    let db = init_db().await;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let roadmap: ImportRoadmap = read_json_file(&path)?;
        let track_id = if let Some(track_slug) = roadmap.track_slug.clone() {
            let mut response = db
                .query("SELECT id FROM tracks WHERE slug = $slug LIMIT 1")
                .bind(("slug", track_slug))
                .await?;
            let row: Option<IdRow> = response.take(0)?;
            row.map(|row| row.id)
        } else {
            None
        };

        let created_by: RecordId = roadmap.created_by.parse()?;
        let status = roadmap.status.unwrap_or_else(|| "draft".to_string());

        let mut roadmap_response = db
            .query(
                "CREATE roadmaps CONTENT { title: $title, slug: $slug, description: $description, image_url: $image_url, track: $track, difficulty: $difficulty, estimated_weeks: $estimated_weeks, status: $status, created_by: $created_by, created_at: time::now(), updated_at: time::now(), deleted: false }",
            )
            .bind(("title", roadmap.title))
            .bind(("slug", roadmap.slug))
            .bind(("description", roadmap.description))
            .bind(("image_url", roadmap.image_url))
            .bind(("track", track_id))
            .bind(("difficulty", roadmap.difficulty))
            .bind(("estimated_weeks", roadmap.estimated_weeks))
            .bind(("status", status))
            .bind(("created_by", created_by))
            .await?;
        let created_roadmap: Option<IdRow> = roadmap_response.take(0)?;
        let roadmap_id = created_roadmap
            .map(|row| row.id)
            .ok_or_else(|| anyhow::anyhow!("Failed to create roadmap"))?;

        for course in roadmap.courses {
            let mut course_response = db
                .query("SELECT id FROM courses WHERE slug = $slug LIMIT 1")
                .bind(("slug", course.course_slug))
                .await?;
            let course_row: Option<IdRow> = course_response.take(0)?;
            let course_id = course_row
                .map(|row| row.id)
                .ok_or_else(|| anyhow::anyhow!("Course not found for roadmap"))?;

            db.query("RELATE $roadmap -> roadmap_courses -> $course SET sort_order = $sort_order, is_required = $is_required, note = $note")
                .bind(("roadmap", roadmap_id.clone()))
                .bind(("course", course_id))
                .bind(("sort_order", course.sort_order))
                .bind(("is_required", course.is_required.unwrap_or(true)))
                .bind(("note", course.note))
                .await?;
        }
    }

    Ok(())
}

#[cfg(feature = "ssr")]
async fn import_frameworks(base: &Path) -> anyhow::Result<()> {
    let mut dir = base.to_path_buf();
    dir.push("frameworks");
    if !dir.exists() {
        anyhow::bail!("frameworks directory not found at {}", dir.display());
    }

    let db = init_db().await;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let framework: ImportFramework = read_json_file(&path)?;
        let track_id = if let Some(track_slug) = framework.track_slug.clone() {
            let mut response = db
                .query("SELECT id FROM tracks WHERE slug = $slug LIMIT 1")
                .bind(("slug", track_slug))
                .await?;
            let row: Option<IdRow> = response.take(0)?;
            row.map(|row| row.id)
        } else {
            None
        };

        let created_by: RecordId = framework.created_by.parse()?;
        let status = framework.status.unwrap_or_else(|| "draft".to_string());

        let mut framework_response = db
            .query(
                "CREATE frameworks CONTENT { title: $title, slug: $slug, description: $description, image_url: $image_url, track: $track, status: $status, created_by: $created_by, created_at: time::now(), updated_at: time::now(), deleted: false }",
            )
            .bind(("title", framework.title))
            .bind(("slug", framework.slug))
            .bind(("description", framework.description))
            .bind(("image_url", framework.image_url))
            .bind(("track", track_id))
            .bind(("status", status))
            .bind(("created_by", created_by))
            .await?;
        let created_framework: Option<IdRow> = framework_response.take(0)?;
        let framework_id = created_framework
            .map(|row| row.id)
            .ok_or_else(|| anyhow::anyhow!("Failed to create framework"))?;

        for milestone in framework.milestones {
            let mut milestone_response = db
                .query("CREATE milestones CONTENT { framework: $framework, title: $title, description: $description, sort_order: $sort_order }")
                .bind(("framework", framework_id.clone()))
                .bind(("title", milestone.title))
                .bind(("description", milestone.description))
                .bind(("sort_order", milestone.sort_order))
                .await?;
            let created_milestone: Option<IdRow> = milestone_response.take(0)?;
            let milestone_id = created_milestone
                .map(|row| row.id)
                .ok_or_else(|| anyhow::anyhow!("Failed to create milestone"))?;

            for course in milestone.courses {
                let mut course_response = db
                    .query("SELECT id FROM courses WHERE slug = $slug LIMIT 1")
                    .bind(("slug", course.course_slug))
                    .await?;
                let course_row: Option<IdRow> = course_response.take(0)?;
                let course_id = course_row
                    .map(|row| row.id)
                    .ok_or_else(|| anyhow::anyhow!("Course not found for milestone"))?;

                db.query("RELATE $milestone -> milestone_courses -> $course SET is_required = $is_required")
                    .bind(("milestone", milestone_id.clone()))
                    .bind(("course", course_id))
                    .bind(("is_required", course.is_required.unwrap_or(true)))
                    .await?;
            }
        }
    }

    Ok(())
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let base = PathBuf::from("scripts/data");

    let run_all = args.iter().any(|arg| arg == "--all");
    if run_all || args.iter().any(|arg| arg == "--tracks") {
        import_tracks(&base).await?;
    }
    if run_all || args.iter().any(|arg| arg == "--courses") {
        import_courses(&base).await?;
    }
    if run_all || args.iter().any(|arg| arg == "--roadmaps") {
        import_roadmaps(&base).await?;
    }
    if run_all || args.iter().any(|arg| arg == "--frameworks") {
        import_frameworks(&base).await?;
    }

    if !run_all
        && !args.iter().any(|arg| {
            arg == "--tracks" || arg == "--courses" || arg == "--roadmaps" || arg == "--frameworks"
        })
    {
        println!(
            "Usage: cargo run --bin import -- --tracks|--courses|--roadmaps|--frameworks|--all"
        );
    }

    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {
    eprintln!("Import CLI requires --features ssr");
}
