use std::path::{Path, PathBuf};
use std::process::Command;

use crate::common::get_test_db_with_ns;
use surrealdb::RecordId;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

/// Copies the minimal seed set (one track, one course, one roadmap and one
/// framework) into a temporary directory so the import runs fast.
fn write_seed_dir(root: &Path) {
    let data_dir = root.join("scripts/data");
    std::fs::create_dir_all(data_dir.join("courses")).expect("failed to create courses dir");
    std::fs::create_dir_all(data_dir.join("roadmaps")).expect("failed to create roadmaps dir");
    std::fs::create_dir_all(data_dir.join("frameworks")).expect("failed to create frameworks dir");

    let source = Path::new("scripts/data");
    std::fs::copy(
        source.join("tracks.json"),
        data_dir.join("tracks.json"),
    )
    .expect("failed to copy tracks.json");
    std::fs::copy(
        source.join("courses/aqeedah-tawheed.json"),
        data_dir.join("courses/aqeedah-tawheed.json"),
    )
    .expect("failed to copy the course seed");
    std::fs::copy(
        source.join("roadmaps/foundations-of-faith.json"),
        data_dir.join("roadmaps/foundations-of-faith.json"),
    )
    .expect("failed to copy the roadmap seed");
    std::fs::copy(
        source.join("frameworks/islamic-studies-framework.json"),
        data_dir.join("frameworks/islamic-studies-framework.json"),
    )
    .expect("failed to copy the framework seed");
    std::fs::copy(
        source.join("educator_ids.json"),
        data_dir.join("educator_ids.json"),
    )
    .expect("failed to copy educator_ids.json");
}

fn seed_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "merzah_seed_test_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock moved backwards")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("failed to create the temp seed dir");
    write_seed_dir(&dir);
    dir
}

fn run_import(cwd: &Path, url: &str, namespace: &str, database: &str) {
    let output = Command::new(env!("CARGO_BIN_EXE_import"))
        .current_dir(cwd)
        .env("SURREAL_URL", url)
        .env("SURREAL_USER", "root")
        .env("SURREAL_PASS", "root")
        .env("SURREAL_NS", namespace)
        .env("SURREAL_DB", database)
        .args(["--all"])
        .output()
        .expect("failed to run the import binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "the import failed\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
}

async fn count(db: &Surreal<Client>, table: &str) -> i64 {
    let mut response = db
        .query(format!("SELECT count() AS count FROM {} GROUP ALL", table))
        .await
        .expect("failed to count records");
    let rows: Vec<CountRow> = response.take(0).expect("failed to count records");
    rows.first().map(|row| row.count).unwrap_or(0)
}

#[derive(Debug, serde::Deserialize)]
struct CountRow {
    count: i64,
}

#[tokio::test]
async fn seeding_twice_updates_records_instead_of_duplicating_them() {
    let (db, namespace, database, port) = get_test_db_with_ns().await;
    let url = format!("127.0.0.1:{}", port);
    let seed = seed_dir();

    // First run: everything is created.
    run_import(&seed, &url, &namespace, &database);

    let courses_after_first = count(&db, "courses").await;
    let modules_after_first = count(&db, "modules").await;
    let lessons_after_first = count(&db, "lessons").await;
    let roadmaps_after_first = count(&db, "roadmaps").await;
    let frameworks_after_first = count(&db, "frameworks").await;
    let milestones_after_first = count(&db, "milestones").await;
    let roadmap_courses_after_first = count(&db, "roadmap_courses").await;
    let milestone_courses_after_first = count(&db, "milestone_courses").await;

    assert_eq!(courses_after_first, 1, "the course should have been created");
    assert!(modules_after_first > 0, "modules should have been created");
    assert!(lessons_after_first > 0, "lessons should have been created");
    assert_eq!(roadmaps_after_first, 1, "the roadmap should have been created");
    assert_eq!(frameworks_after_first, 1, "the framework should have been created");

    // Change a course in the database to prove that the second run updates the
    // existing record instead of adding a second one.
    db.query("UPDATE courses SET title = 'Stale title', description = 'Stale description'")
        .await
        .expect("failed to change the stored course");

    // Second run with the same seed data.
    run_import(&seed, &url, &namespace, &database);

    assert_eq!(
        count(&db, "courses").await,
        courses_after_first,
        "re-running the importer must not create a second course"
    );
    assert_eq!(
        count(&db, "modules").await,
        modules_after_first,
        "re-running the importer must not create duplicate modules"
    );
    assert_eq!(
        count(&db, "lessons").await,
        lessons_after_first,
        "re-running the importer must not create duplicate lessons"
    );
    assert_eq!(
        count(&db, "roadmaps").await,
        roadmaps_after_first,
        "re-running the importer must not create a second roadmap"
    );
    assert_eq!(
        count(&db, "frameworks").await,
        frameworks_after_first,
        "re-running the importer must not create a second framework"
    );
    assert_eq!(
        count(&db, "milestones").await,
        milestones_after_first,
        "re-running the importer must not create duplicate milestones"
    );
    assert_eq!(
        count(&db, "roadmap_courses").await,
        roadmap_courses_after_first,
        "re-running the importer must not create duplicate roadmap links"
    );
    assert_eq!(
        count(&db, "milestone_courses").await,
        milestone_courses_after_first,
        "re-running the importer must not create duplicate milestone links"
    );

    // The stale title must have been replaced by the seed value again.
    let mut response = db
        .query("SELECT title FROM courses LIMIT 1")
        .await
        .expect("failed to read the course back");
    let rows: Vec<TitleRow> = response.take(0).expect("failed to read the course back");
    let title = rows
        .first()
        .map(|row| row.title.clone())
        .expect("the course should still exist");
    assert_ne!(
        title, "Stale title",
        "the existing course should have been updated with the seed data"
    );

    std::fs::remove_dir_all(&seed).ok();
}

#[derive(Debug, serde::Deserialize)]
struct TitleRow {
    title: String,
}

#[tokio::test]
async fn seeding_creates_the_educator_account_the_courses_point_at() {
    let (db, namespace, database, port) = get_test_db_with_ns().await;
    let url = format!("127.0.0.1:{}", port);
    let seed = seed_dir();

    run_import(&seed, &url, &namespace, &database);

    let mut response = db
        .query("SELECT VALUE educator FROM courses LIMIT 1")
        .await
        .expect("failed to read the educator link");
    let educators: Vec<RecordId> = response.take(0).expect("failed to read the educator link");
    let educator = educators
        .first()
        .map(|id| id.to_string())
        .expect("the course should link to an educator");

    let mut user_response = db
        .query(format!("SELECT count() AS count FROM users WHERE id = {} GROUP ALL", educator))
        .await
        .expect("failed to look up the educator account");
    let rows: Vec<CountRow> = user_response.take(0).expect("failed to look up the educator account");
    assert_eq!(
        rows.first().map(|row| row.count).unwrap_or(0),
        1,
        "the educator account {} should exist so the link resolves",
        educator
    );

    std::fs::remove_dir_all(&seed).ok();
}

#[tokio::test]
async fn seeding_adds_new_lessons_and_soft_deletes_removed_ones() {
    let (db, namespace, database, port) = get_test_db_with_ns().await;
    let url = format!("127.0.0.1:{}", port);
    let seed = seed_dir();
    let course_file = seed.join("scripts/data/courses/aqeedah-tawheed.json");

    run_import(&seed, &url, &namespace, &database);
    let lessons_after_first = count(&db, "lessons").await;
    assert!(lessons_after_first > 0, "lessons should have been created");

    // Add a lesson to the seed file and seed again: the new lesson has to
    // appear, and the existing ones must not be duplicated.
    let mut course: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&course_file).expect("failed to read the course seed"),
    )
    .expect("failed to parse the course seed");
    let modules = course["modules"]
        .as_array_mut()
        .expect("the course seed should have modules");
    let template = modules[0]["lessons"][0].clone();
    let mut new_lesson = template;
    new_lesson["title"] = serde_json::Value::String("A Brand New Lesson".to_string());
    new_lesson["sort_order"] = serde_json::Value::from(999);
    modules[0]["lessons"]
        .as_array_mut()
        .expect("the module should have lessons")
        .push(new_lesson);
    std::fs::write(
        &course_file,
        serde_json::to_string_pretty(&course).expect("failed to write the course seed"),
    )
    .expect("failed to write the course seed");

    run_import(&seed, &url, &namespace, &database);
    assert_eq!(
        count(&db, "lessons").await,
        lessons_after_first + 1,
        "the added lesson should have been created without duplicating the others"
    );

    // Remove the lesson again: it has to be soft-deleted, not orphaned.
    let mut course: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&course_file).expect("failed to read the course seed"),
    )
    .expect("failed to parse the course seed");
    if let Some(modules) = course["modules"].as_array_mut() {
        for module in modules.iter_mut() {
            if let Some(lessons) = module["lessons"].as_array_mut() {
                lessons.retain(|lesson| {
                    lesson["title"].as_str() != Some("A Brand New Lesson")
                });
            }
        }
    }
    std::fs::write(
        &course_file,
        serde_json::to_string_pretty(&course).expect("failed to write the course seed"),
    )
    .expect("failed to write the course seed");

    run_import(&seed, &url, &namespace, &database);

    let mut response = db
        .query("SELECT count() AS count FROM lessons WHERE deleted = true GROUP ALL")
        .await
        .expect("failed to count the removed lessons");
    let rows: Vec<CountRow> = response.take(0).expect("failed to count the removed lessons");
    assert_eq!(
        rows.first().map(|row| row.count).unwrap_or(0),
        1,
        "the removed lesson should have been soft-deleted"
    );

    // The course lesson_count only counts the active lessons.
    let mut response = db
        .query("SELECT lesson_count FROM courses LIMIT 1")
        .await
        .expect("failed to read the lesson count");
    let rows: Vec<LessonCountRow> = response.take(0).expect("failed to read the lesson count");
    assert_eq!(
        rows.first().map(|row| row.lesson_count).unwrap_or(0),
        lessons_after_first as i64,
        "the lesson count should only include the active lessons"
    );

    std::fs::remove_dir_all(&seed).ok();
}

#[derive(Debug, serde::Deserialize)]
struct LessonCountRow {
    lesson_count: i64,
}

#[tokio::test]
async fn the_python_importer_does_not_duplicate_records_either() {
    let (db, namespace, database, port) = get_test_db_with_ns().await;
    let url = format!("127.0.0.1:{}", port);
    let seed = seed_dir();
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/import_db.py");

    let run_python = || {
        let output = Command::new("python3")
            .current_dir(&seed)
            .env("SURREAL_URL", &url)
            .env("SURREAL_USER", "root")
            .env("SURREAL_PASS", "root")
            .env("SURREAL_NS", &namespace)
            .env("SURREAL_DB", &database)
            .arg(&script)
            .arg("--all")
            .output()
            .expect("failed to run the python importer");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success(),
            "the python importer failed\nstdout:\n{}\nstderr:\n{}",
            stdout,
            stderr
        );
    };

    run_python();
    let courses_after_first = count(&db, "courses").await;
    let lessons_after_first = count(&db, "lessons").await;
    let roadmap_courses_after_first = count(&db, "roadmap_courses").await;

    run_python();

    assert_eq!(
        count(&db, "courses").await,
        courses_after_first,
        "re-running the python importer must not create a second course"
    );
    assert_eq!(
        count(&db, "lessons").await,
        lessons_after_first,
        "re-running the python importer must not create duplicate lessons"
    );
    assert_eq!(
        count(&db, "roadmap_courses").await,
        roadmap_courses_after_first,
        "re-running the python importer must not create duplicate roadmap links"
    );

    std::fs::remove_dir_all(&seed).ok();
}
