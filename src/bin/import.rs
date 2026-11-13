#[cfg(feature = "ssr")]
use std::collections::HashMap;
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
use merzah::models::education::{
    CourseRecord, CourseStatus, LessonRecord, ModuleRecord, TrackRecord,
};
#[cfg(feature = "ssr")]
use merzah::models::import::{ImportCourse, ImportFramework, ImportRoadmap, ImportTrack};
#[cfg(feature = "ssr")]
use merzah::services::course_stats::update_course_lesson_count;

// ============================================================
// Idempotent seeding
// ============================================================
// Every entity is matched by its natural key (track/course/roadmap/
// framework slug, module/lesson title within its parent, relation
// in/out pair) so re-running the importer updates the existing
// records instead of creating duplicates.
//
//   * course / track / roadmap / framework  -> keyed by slug
//   * module                                -> keyed by (course, title)
//   * lesson                                -> keyed by (module, title)
//   * roadmap_courses / milestone_courses    -> keyed by (in, out)
//   * educators                             -> keyed by record id
//
// Modules/lessons that disappear from the seed files are soft-deleted
// (deleted = true) instead of removed, so user progress rows that
// reference them stay valid.
// ============================================================

#[cfg(feature = "ssr")]
const UNUSABLE_PASSWORD_HASH: &str = "UNUSABLE_SEED_ACCOUNT";

/// Record id used when a seed file has no real creator (see import_db.py).
#[cfg(feature = "ssr")]
const CREATED_BY_FALLBACK: &str = "users:lyp8kc57il37w8gt38q7";

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
#[derive(Debug, Deserialize)]
struct EdgeRow {
    pub id: RecordId,
}

/// name -> id pairs for the educator accounts used by the seed files.
#[cfg(feature = "ssr")]
fn educator_names(base: &Path) -> HashMap<String, String> {
    let mut path = base.to_path_buf();
    path.push("educator_ids.json");
    let mut names = HashMap::new();
    if !path.exists() {
        return names;
    }
    if let Ok(rows) = read_json_file::<Vec<HashMap<String, String>>>(&path) {
        for row in rows {
            if let (Some(name), Some(id)) = (row.get("name"), row.get("id")) {
                names.insert(id.clone(), name.clone());
            }
        }
    }
    names
}

/// Makes sure an educator account exists for `rid` so course links resolve.
///
/// Returns true when a new account was created.
#[cfg(feature = "ssr")]
async fn ensure_educator(
    db: &surrealdb::Surreal<surrealdb::engine::remote::ws::Client>,
    rid: &RecordId,
    display_name: Option<&str>,
) -> bool {
    let mut response = db
        .query("SELECT id FROM users WHERE id = $rid LIMIT 1")
        .bind(("rid", rid.clone()))
        .await
        .expect("educator lookup failed");
    let existing: Option<IdRow> = response.take(0).expect("educator lookup failed");
    if existing.is_some() {
        return false;
    }

    let name = match display_name {
        Some(name) if !name.trim().is_empty() => name.to_string(),
        _ => format!("Educator {}", rid),
    };
    db.query(
        "UPSERT $rid CONTENT { display_name: $name, password_hash: $hash, role: 'educator', created_at: time::now(), updated_at: time::now() }",
    )
    .bind(("rid", rid.clone()))
    .bind(("name", name))
    .bind(("hash", UNUSABLE_PASSWORD_HASH))
    .await
    .expect("failed to create educator account");
    true
}

#[cfg(feature = "ssr")]
async fn ensure_track(
    db: &surrealdb::Surreal<surrealdb::engine::remote::ws::Client>,
    track: &ImportTrack,
) -> RecordId {
    let mut response = db
        .query("SELECT id FROM tracks WHERE slug = $slug LIMIT 1")
        .bind(("slug", track.slug.clone()))
        .await
        .expect("track lookup failed");
    let existing: Option<IdRow> = response.take(0).expect("track lookup failed");
    if let Some(row) = existing {
        db.query(
            "UPDATE $id SET name = $name, description = $description, icon = $icon, image_url = $image_url, sort_order = $sort_order, deleted = false, updated_at = time::now()",
        )
        .bind(("id", row.id.clone()))
        .bind(("name", track.name.clone()))
        .bind(("description", track.description.clone()))
        .bind(("icon", track.icon.clone()))
        .bind(("image_url", track.image_url.clone()))
        .bind(("sort_order", track.sort_order))
        .await
        .expect("failed to update track");
        return row.id;
    }

    let now: Datetime = Utc::now().into();
    let record = TrackRecord::new(
        track.name.clone(),
        track.slug.clone(),
        track.description.clone(),
        track.icon.clone(),
        track.image_url.clone(),
        track.sort_order,
        now.clone(),
        now,
        false,
    );
    let mut response = db
        .query("CREATE tracks CONTENT $track")
        .bind(("track", record))
        .await
        .expect("failed to create track");
    let created: Option<IdRow> = response.take(0).expect("failed to create track");
    created
        .map(|row| row.id)
        .expect("failed to create track")
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
    for track in &tracks {
        let id = ensure_track(&db, track).await;
        println!("  = track {} ({})", track.slug, id);
    }

    Ok(())
}

#[cfg(feature = "ssr")]
async fn import_courses(base: &Path) -> anyhow::Result<()> {
    let mut dir: PathBuf = base.to_path_buf();
    dir.push("courses");
    if !dir.exists() {
        anyhow::bail!("courses directory not found at {}", dir.display());
    }

    let db = init_db().await;
    let educator_display_names = educator_names(base);

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
            .await?.check()?;
        let track_row: Option<IdRow> = track_response.take(0)?;
        let track_id = track_row
            .map(|row| row.id)
            .ok_or_else(|| anyhow::anyhow!("Track not found for slug {}", course.track_slug))?;

        let educator_id: RecordId = course.educator_id.parse()?;
        let educator_name = educator_display_names.get(&course.educator_id).cloned();
        if ensure_educator(&db, &educator_id, educator_name.as_deref()).await {
            println!(
                "  + educator account {} created for {}",
                educator_id, course.educator_id
            );
        }

        // Course is keyed by slug: update the existing record instead of
        // creating a second one.
        let mut course_response = db
            .query("SELECT id FROM courses WHERE slug = $slug LIMIT 1")
            .bind(("slug", course.slug.clone()))
            .await?.check()?;
        let existing_course: Option<IdRow> = course_response.take(0)?;

        let course_id = match existing_course {
            Some(row) => {
                db.query(
                    "UPDATE $id SET title = $title, description = $description, short_description = $short_description, track = $track, educator = $educator, level = $level, status = $status, language = $language, thumbnail_url = $thumbnail_url, video_url = $video_url, deleted = false, updated_at = time::now()",
                )
                .bind(("id", row.id.clone()))
                .bind(("title", course.title.clone()))
                .bind(("description", course.description.clone()))
                .bind(("short_description", course.short_description.clone()))
                .bind(("track", track_id.clone()))
                .bind(("educator", educator_id.clone()))
                .bind(("level", course.level.clone()))
                .bind(("status", CourseStatus::Published))
                .bind(("language", course.language.clone().unwrap_or_else(|| "en".to_string())))
                .bind(("thumbnail_url", course.thumbnail_url.clone()))
                .bind(("video_url", course.video_url.clone()))
                .await?.check()?;
                println!("  = course {} updated", course.slug);
                row.id
            }
            None => {
                let now: Datetime = Utc::now().into();
                let record = CourseRecord::new(
                    course.title.clone(),
                    course.slug.clone(),
                    course.description.clone(),
                    course.short_description.clone(),
                    track_id.clone(),
                    educator_id.clone(),
                    course.level.clone(),
                    CourseStatus::Published,
                    course.language.clone().unwrap_or_else(|| "en".to_string()),
                    course.thumbnail_url.clone(),
                    course.video_url.clone(),
                    0,
                    0,
                    0,
                    now.clone(),
                    now,
                    false,
                );
                let mut response = db
                    .query("CREATE courses CONTENT $course")
                    .bind(("course", record))
                    .await?.check()?;
                let created: Option<IdRow> = response.take(0)?;
                let id = created
                    .map(|row| row.id)
                    .ok_or_else(|| anyhow::anyhow!("Failed to create course"))?;
                println!("  + course {} created", course.slug);
                id
            }
        };

        // Modules are keyed by (course, title).
        let mut kept_module_ids: Vec<RecordId> = Vec::new();
        for module in course.modules {
            let mut module_response = db
                .query("SELECT id FROM modules WHERE course = $course AND title = $title LIMIT 1")
                .bind(("course", course_id.clone()))
                .bind(("title", module.title.clone()))
                .await?.check()?;
            let existing_module: Option<IdRow> = module_response.take(0)?;

            let module_id = match existing_module {
                Some(row) => {
                    db.query(
                        "UPDATE $id SET description = $description, sort_order = $sort_order, deleted = false, updated_at = time::now()",
                    )
                    .bind(("id", row.id.clone()))
                    .bind(("description", module.description.clone()))
                    .bind(("sort_order", module.sort_order.unwrap_or(0)))
                    .await?.check()?;
                    row.id
                }
                None => {
                    let now: Datetime = Utc::now().into();
                    let module_record = ModuleRecord::new(
                        module.title.clone(),
                        course_id.clone(),
                        module.description.clone(),
                        module.sort_order.unwrap_or(0),
                        now.clone(),
                        now,
                        false,
                    );
                    let mut response = db
                        .query("CREATE modules CONTENT $module")
                        .bind(("module", module_record))
                        .await?.check()?;
                    let created: Option<IdRow> = response.take(0)?;
                    created
                        .map(|row| row.id)
                        .ok_or_else(|| anyhow::anyhow!("Failed to create module"))?
                }
            };
            kept_module_ids.push(module_id.clone());

            // Lessons are keyed by (module, title).
            let mut kept_lesson_ids: Vec<RecordId> = Vec::new();
            for lesson in module.lessons {
                let mut lesson_response = db
                    .query("SELECT id FROM lessons WHERE module = $module AND title = $title LIMIT 1")
                    .bind(("module", module_id.clone()))
                    .bind(("title", lesson.title.clone()))
                    .await?.check()?;
                let existing_lesson: Option<IdRow> = lesson_response.take(0)?;

                let lesson_id = match existing_lesson {
                    Some(row) => {
                        db.query(
                            "UPDATE $id SET content_type = $content_type, content = $content, video_url = $video_url, video_duration_seconds = $video_duration_seconds, audio_url = $audio_url, pdf_url = $pdf_url, external_url = $external_url, thumbnail_url = $thumbnail_url, duration_minutes = $duration_minutes, sort_order = $sort_order, is_preview = $is_preview, deleted = false, updated_at = time::now()",
                        )
                        .bind(("id", row.id.clone()))
                        .bind(("content_type", lesson.content_type.clone()))
                        .bind(("content", lesson.content.clone()))
                        .bind(("video_url", lesson.video_url.clone()))
                        .bind(("video_duration_seconds", lesson.video_duration_seconds))
                        .bind(("audio_url", lesson.audio_url.clone()))
                        .bind(("pdf_url", lesson.pdf_url.clone()))
                        .bind(("external_url", lesson.external_url.clone()))
                        .bind(("thumbnail_url", lesson.thumbnail_url.clone()))
                        .bind(("duration_minutes", lesson.duration_minutes.unwrap_or(5)))
                        .bind(("sort_order", lesson.sort_order.unwrap_or(0)))
                        .bind(("is_preview", lesson.is_preview.unwrap_or(false)))
                        .await?.check()?;
                        row.id
                    }
                    None => {
                        let now: Datetime = Utc::now().into();
                        let lesson_record = LessonRecord::new(
                            lesson.title.clone(),
                            module_id.clone(),
                            lesson.content_type,
                            lesson.content,
                            lesson.video_url,
                            lesson.video_duration_seconds,
                            lesson.audio_url,
                            lesson.pdf_url,
                            lesson.external_url,
                            lesson.thumbnail_url,
                            lesson.duration_minutes.unwrap_or(5),
                            lesson.sort_order.unwrap_or(0),
                            lesson.is_preview.unwrap_or(false),
                            now.clone(),
                            now,
                            false,
                        );
                        db.query("CREATE lessons CONTENT $lesson")
                            .bind(("lesson", lesson_record))
                            .await?.check()?;
                        // Fetch the created id so stale-lesson cleanup can see it.
                        let mut created_response = db
                            .query(
                                "SELECT id FROM lessons WHERE module = $module AND title = $title LIMIT 1",
                            )
                            .bind(("module", module_id.clone()))
                            .bind(("title", lesson.title.clone()))
                            .await?.check()?;
                        let created: Option<IdRow> = created_response.take(0)?;
                        created
                            .map(|row| row.id)
                            .ok_or_else(|| anyhow::anyhow!("Failed to create lesson"))?
                    }
                };
                kept_lesson_ids.push(lesson_id);
            }

            // Lessons that are no longer part of the seed are soft-deleted so
            // existing progress rows keep pointing at valid records.
            let mut stale_response = db
                .query("SELECT id FROM lessons WHERE module = $module AND deleted = false")
                .bind(("module", module_id.clone()))
                .await?.check()?;
            let stale: Vec<IdRow> = stale_response.take(0)?;
            for row in stale {
                if !kept_lesson_ids.contains(&row.id) {
                    db.query("UPDATE $id SET deleted = true, updated_at = time::now()")
                        .bind(("id", row.id))
                        .await?.check()?;
                }
            }
        }

        // Same for modules that dropped out of the seed.
        let mut stale_module_response = db
            .query("SELECT id FROM modules WHERE course = $course AND deleted = false")
            .bind(("course", course_id.clone()))
            .await?.check()?;
        let stale_modules: Vec<IdRow> = stale_module_response.take(0)?;
        for row in stale_modules {
            if !kept_module_ids.contains(&row.id) {
                db.query("UPDATE $id SET deleted = true, updated_at = time::now()")
                    .bind(("id", row.id))
                    .await?.check()?;
            }
        }

        let _ = update_course_lesson_count(&course_id, &db).await;
    }

    Ok(())
}

/// Resolves the creator of a roadmap/framework, tolerating the "PLACEHOLDER"
/// value used by the seed files (the creator is not part of the data).
#[cfg(feature = "ssr")]
fn resolve_created_by(raw: &str) -> RecordId {
    match raw.parse::<RecordId>() {
        Ok(id) => id,
        Err(_) => {
            println!("  ! placeholder creator, using {CREATED_BY_FALLBACK}");
            CREATED_BY_FALLBACK.parse().expect("invalid fallback record id")
        }
    }
}


/// Renders a record id coming back from the database as a comparable string.
/// SurrealDB returns record ids either as "table:id" or as
/// { "tb": "table", "id": "id" } / { "tb": "table", "id": { "String": "id" } }.
#[cfg(feature = "ssr")]
fn record_id_string(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(raw) => Some(raw.clone()),
        serde_json::Value::Object(map) => {
            let table = map.get("tb")?.as_str()?;
            let id = map.get("id")?;
            let id = match id {
                serde_json::Value::String(raw) => raw.clone(),
                serde_json::Value::Object(inner) => inner.get("String")?.as_str()?.to_string(),
                _ => return None,
            };
            Some(format!("{}:{}", table, id))
        }
        _ => None,
    }
}

/// Syncs one relation edge: updates it when it already exists, creates it
/// otherwise, and removes edges that are no longer part of the seed.
#[cfg(feature = "ssr")]
async fn sync_relation(
    db: &surrealdb::Surreal<surrealdb::engine::remote::ws::Client>,
    table: &str,
    parent: &RecordId,
    child: &RecordId,
    fields: Vec<(&str, serde_json::Value)>,
) {
    let mut response = db
        .query(&format!(
            "SELECT id FROM {table} WHERE in = $parent AND out = $child LIMIT 1"
        ))
        .bind(("parent", parent.clone()))
        .bind(("child", child.clone()))
        .await
        .expect("relation lookup failed");
    let existing: Option<EdgeRow> = response.take(0).expect("relation lookup failed");

    if let Some(row) = existing {
        if fields.is_empty() {
            return;
        }
        let assignments: Vec<String> = fields
            .iter()
            .map(|(key, value)| format!("{} = {}", key, value))
            .collect();
        db.query(&format!(
            "UPDATE $id SET {} , updated_at = time::now()",
            assignments.join(", ")
        ))
        .bind(("id", row.id))
        .await
        .expect("failed to update relation");
        return;
    }

    let assignments: Vec<String> = fields
        .iter()
        .map(|(key, value)| format!("{} = {}", key, value))
        .collect();
    let mut statement = format!("RELATE $parent -> {} -> $child", table);
    if !assignments.is_empty() {
        statement.push_str(&format!(" SET {}", assignments.join(", ")));
    }
    db.query(&statement)
        .bind(("parent", parent.clone()))
        .bind(("child", child.clone()))
        .await
        .expect("failed to create relation");
}

#[cfg(feature = "ssr")]
async fn import_roadmaps(base: &Path) -> anyhow::Result<()> {
    let mut dir: PathBuf = base.to_path_buf();
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
                .await?.check()?;
            let row: Option<IdRow> = response.take(0)?;
            row.map(|row| row.id)
        } else {
            None
        };

        let created_by = resolve_created_by(&roadmap.created_by);
        let status = roadmap.status.unwrap_or_else(|| "draft".to_string());

        // Roadmaps are keyed by slug.
        let mut existing_response = db
            .query("SELECT id FROM roadmaps WHERE slug = $slug LIMIT 1")
            .bind(("slug", roadmap.slug.clone()))
            .await?.check()?;
        let existing: Option<IdRow> = existing_response.take(0)?;

        let roadmap_id = match existing {
            Some(row) => {
                db.query(
                    "UPDATE $id SET title = $title, description = $description, image_url = $image_url, track = $track, difficulty = $difficulty, estimated_weeks = $estimated_weeks, status = $status, created_by = $created_by, deleted = false, updated_at = time::now()",
                )
                .bind(("id", row.id.clone()))
                .bind(("title", roadmap.title.clone()))
                .bind(("description", roadmap.description.clone()))
                .bind(("image_url", roadmap.image_url.clone()))
                .bind(("track", track_id.clone()))
                .bind(("difficulty", roadmap.difficulty.clone()))
                .bind(("estimated_weeks", roadmap.estimated_weeks))
                .bind(("status", status.clone()))
                .bind(("created_by", created_by.clone()))
                .await?.check()?;
                println!("  = roadmap {} updated", roadmap.slug);
                row.id
            }
            None => {
                let mut response = db
                    .query(
                        "CREATE roadmaps CONTENT { title: $title, slug: $slug, description: $description, image_url: $image_url, track: $track, difficulty: $difficulty, estimated_weeks: $estimated_weeks, status: $status, created_by: $created_by, created_at: time::now(), updated_at: time::now(), deleted: false }",
                    )
                    .bind(("title", roadmap.title.clone()))
                    .bind(("slug", roadmap.slug.clone()))
                    .bind(("description", roadmap.description.clone()))
                    .bind(("image_url", roadmap.image_url.clone()))
                    .bind(("track", track_id.clone()))
                    .bind(("difficulty", roadmap.difficulty.clone()))
                    .bind(("estimated_weeks", roadmap.estimated_weeks))
                    .bind(("status", status.clone()))
                    .bind(("created_by", created_by.clone()))
                    .await?.check()?;
                let created: Option<IdRow> = response.take(0)?;
                let id = created
                    .map(|row| row.id)
                    .ok_or_else(|| anyhow::anyhow!("Failed to create roadmap"))?;
                println!("  + roadmap {} created", roadmap.slug);
                id
            }
        };

        let mut kept_course_ids: Vec<RecordId> = Vec::new();
        for course in roadmap.courses {
            let mut course_response = db
                .query("SELECT id FROM courses WHERE slug = $slug LIMIT 1")
                .bind(("slug", course.course_slug.clone()))
                .await?.check()?;
            let course_row: Option<IdRow> = course_response.take(0)?;
            let course_id = course_row
                .map(|row| row.id)
                .ok_or_else(|| anyhow::anyhow!("Course not found for roadmap"))?;

            sync_relation(
                &db,
                "roadmap_courses",
                &roadmap_id,
                &course_id,
                vec![
                    ("sort_order", serde_json::json!(course.sort_order)),
                    ("is_required", serde_json::json!(course.is_required.unwrap_or(true))),
                    ("note", serde_json::json!(course.note)),
                ],
            )
            .await;
            kept_course_ids.push(course_id);
        }

        // Drop edges for courses that are no longer part of the roadmap.
        let mut edge_response = db
            .query("SELECT id, out FROM roadmap_courses WHERE in = $roadmap")
            .bind(("roadmap", roadmap_id.clone()))
            .await?.check()?;
        let edges: Vec<HashMap<String, serde_json::Value>> = edge_response.take(0)?;
        for edge in edges {
            let out = edge.get("out").and_then(record_id_string);
            let keep = match out {
                Some(out) => kept_course_ids
                    .iter()
                    .any(|id| id.to_string() == out),
                None => true,
            };
            if !keep {
                if let Some(edge_id) = edge.get("id").and_then(record_id_string) {
                    db.query("DELETE $id").bind(("id", edge_id)).await?.check()?;
                }
            }
        }
    }

    Ok(())
}

#[cfg(feature = "ssr")]
async fn import_frameworks(base: &Path) -> anyhow::Result<()> {
    let mut dir: PathBuf = base.to_path_buf();
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
                .await?.check()?;
            let row: Option<IdRow> = response.take(0)?;
            row.map(|row| row.id)
        } else {
            None
        };

        let created_by = resolve_created_by(&framework.created_by);
        let status = framework.status.unwrap_or_else(|| "draft".to_string());

        // Frameworks are keyed by slug.
        let mut existing_response = db
            .query("SELECT id FROM frameworks WHERE slug = $slug LIMIT 1")
            .bind(("slug", framework.slug.clone()))
            .await?.check()?;
        let existing: Option<IdRow> = existing_response.take(0)?;

        let framework_id = match existing {
            Some(row) => {
                db.query(
                    "UPDATE $id SET title = $title, description = $description, image_url = $image_url, track = $track, status = $status, created_by = $created_by, deleted = false, updated_at = time::now()",
                )
                .bind(("id", row.id.clone()))
                .bind(("title", framework.title.clone()))
                .bind(("description", framework.description.clone()))
                .bind(("image_url", framework.image_url.clone()))
                .bind(("track", track_id.clone()))
                .bind(("status", status.clone()))
                .bind(("created_by", created_by.clone()))
                .await?.check()?;
                println!("  = framework {} updated", framework.slug);
                row.id
            }
            None => {
                let mut response = db
                    .query(
                        "CREATE frameworks CONTENT { title: $title, slug: $slug, description: $description, image_url: $image_url, track: $track, status: $status, created_by: $created_by, created_at: time::now(), updated_at: time::now(), deleted: false }",
                    )
                    .bind(("title", framework.title.clone()))
                    .bind(("slug", framework.slug.clone()))
                    .bind(("description", framework.description.clone()))
                    .bind(("image_url", framework.image_url.clone()))
                    .bind(("track", track_id.clone()))
                    .bind(("status", status.clone()))
                    .bind(("created_by", created_by.clone()))
                    .await?.check()?;
                let created: Option<IdRow> = response.take(0)?;
                let id = created
                    .map(|row| row.id)
                    .ok_or_else(|| anyhow::anyhow!("Failed to create framework"))?;
                println!("  + framework {} created", framework.slug);
                id
            }
        };

        // Milestones are keyed by (framework, title).
        let mut kept_milestone_ids: Vec<RecordId> = Vec::new();
        for milestone in framework.milestones {
            let mut milestone_response = db
                .query("SELECT id FROM milestones WHERE framework = $framework AND title = $title LIMIT 1")
                .bind(("framework", framework_id.clone()))
                .bind(("title", milestone.title.clone()))
                .await?.check()?;
            let existing_milestone: Option<IdRow> = milestone_response.take(0)?;

            let milestone_id = match existing_milestone {
                Some(row) => {
                    db.query("UPDATE $id SET description = $description, sort_order = $sort_order")
                        .bind(("id", row.id.clone()))
                        .bind(("description", milestone.description.clone()))
                        .bind(("sort_order", milestone.sort_order))
                        .await?.check()?;
                    row.id
                }
                None => {
                    let mut response = db
                        .query("CREATE milestones CONTENT { framework: $framework, title: $title, description: $description, sort_order: $sort_order }")
                        .bind(("framework", framework_id.clone()))
                        .bind(("title", milestone.title.clone()))
                        .bind(("description", milestone.description.clone()))
                        .bind(("sort_order", milestone.sort_order))
                        .await?.check()?;
                    let created: Option<IdRow> = response.take(0)?;
                    created
                        .map(|row| row.id)
                        .ok_or_else(|| anyhow::anyhow!("Failed to create milestone"))?
                }
            };
            kept_milestone_ids.push(milestone_id.clone());

            let mut kept_course_ids: Vec<RecordId> = Vec::new();
            for course in milestone.courses {
                let mut course_response = db
                    .query("SELECT id FROM courses WHERE slug = $slug LIMIT 1")
                    .bind(("slug", course.course_slug.clone()))
                    .await?.check()?;
                let course_row: Option<IdRow> = course_response.take(0)?;
                let course_id = course_row
                    .map(|row| row.id)
                    .ok_or_else(|| anyhow::anyhow!("Course not found for milestone"))?;

                sync_relation(
                    &db,
                    "milestone_courses",
                    &milestone_id,
                    &course_id,
                    vec![(
                        "is_required",
                        serde_json::json!(course.is_required.unwrap_or(true)),
                    )],
                )
                .await;
                kept_course_ids.push(course_id);
            }

            let mut edge_response = db
                .query("SELECT id, out FROM milestone_courses WHERE in = $milestone")
                .bind(("milestone", milestone_id.clone()))
                .await?.check()?;
            let edges: Vec<HashMap<String, serde_json::Value>> = edge_response.take(0)?;
            for edge in edges {
                let out = edge.get("out").and_then(record_id_string);
                let keep = match out {
                    Some(out) => kept_course_ids.iter().any(|id| id.to_string() == out),
                    None => true,
                };
                if !keep {
                    if let Some(edge_id) = edge.get("id").and_then(record_id_string) {
                        db.query("DELETE $id").bind(("id", edge_id)).await?.check()?;
                    }
                }
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
