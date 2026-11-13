#!/usr/bin/env python3
"""Seed SurrealDB with tracks/courses via the HTTP API.

Same behavior as `cargo run --bin import --features ssr -- ...`, but needs
no Rust compilation (stdlib only, tiny RAM footprint).

Usage:
    python3 scripts/import_db.py --tracks
    python3 scripts/import_db.py --courses
    python3 scripts/import_db.py --tracks --courses

Config comes from the environment, falling back to the repo .env file
(SURREAL_URL, SURREAL_USER, SURREAL_PASS, SURREAL_NS, SURREAL_DB).

Re-runs are safe: every record is matched by its natural key and updated in
place instead of being inserted again, so running the importer twice never
produces duplicates.

    * track / course / roadmap / framework -> keyed by slug
    * module                               -> keyed by (course, title)
    * lesson                               -> keyed by (module, title)
    * roadmap_courses / milestone_courses   -> keyed by (in, out)
    * educator accounts                    -> keyed by record id

Modules and lessons that disappeared from the seed files are soft-deleted
(deleted = true) instead of removed, so progress rows that reference them stay
valid.

NOTE: tracks are written with created_at/updated_at/deleted fields, which the
Rust importer omits. They are required because the app reads tracks with
`WHERE deleted = false`, which filters out records lacking that field.
"""

import base64
import json
import os
import sys
import urllib.error
import urllib.request

REPO_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DATA_DIR = os.path.join(REPO_DIR, "scripts", "data")
ENV_FILE = os.path.join(REPO_DIR, ".env")


def load_dotenv(path):
    try:
        with open(path, encoding="utf-8") as fh:
            for line in fh:
                line = line.strip()
                if not line or line.startswith("#") or "=" not in line:
                    continue
                key, value = line.split("=", 1)
                key = key.strip()
                value = value.strip()
                if len(value) >= 2 and value[0] == value[-1] and value[0] in "\"'":
                    value = value[1:-1]
                os.environ.setdefault(key, value)
    except FileNotFoundError:
        pass


load_dotenv(ENV_FILE)

SURREAL_URL = os.environ.get("SURREAL_URL", "http://127.0.0.1:5100").rstrip("/")
if "://" not in SURREAL_URL:
    # .env may hold a bare host:port (fine for the Rust WS client);
    # the HTTP API needs an explicit scheme.
    SURREAL_URL = "http://" + SURREAL_URL
SURREAL_USER = os.environ.get("SURREAL_USER", "")
SURREAL_PASS = os.environ.get("SURREAL_PASS", "")
SURREAL_NS = os.environ.get("SURREAL_NS", "")
SURREAL_DB = os.environ.get("SURREAL_DB", "")

for name in ("SURREAL_USER", "SURREAL_PASS", "SURREAL_NS", "SURREAL_DB"):
    if not os.environ.get(name):
        sys.exit("error: %s must be set (env or %s)" % (name, ENV_FILE))

# Seed files use "PLACEHOLDER" for created_by (the Rust importer would fail
# parsing that as a record id). Fall back to User_711rjv instead.
CREATED_BY_FALLBACK = "users:lyp8kc57il37w8gt38q7"


def resolve_created_by(value, what):
    if value and value != "PLACEHOLDER":
        return value
    print(
        "  ! %s has placeholder creator, using %s" % (what, CREATED_BY_FALLBACK),
        flush=True,
    )
    return CREATED_BY_FALLBACK


def sql(query):
    """Run one SQL statement, return its result payload (raise on error)."""
    creds = base64.b64encode(
        ("%s:%s" % (SURREAL_USER, SURREAL_PASS)).encode("utf-8")
    ).decode("ascii")
    req = urllib.request.Request(
        SURREAL_URL + "/sql",
        data=query.encode("utf-8"),
        method="POST",
        headers={
            "Accept": "application/json",
            "Content-Type": "text/plain",
            "Authorization": "Basic " + creds,
            # v2 names + legacy names (accepted by both v1 and v2)
            "Surreal-NS": SURREAL_NS,
            "Surreal-DB": SURREAL_DB,
            "NS": SURREAL_NS,
            "DB": SURREAL_DB,
        },
    )
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            payload = json.loads(resp.read().decode("utf-8"))
    except urllib.error.HTTPError as exc:
        body = exc.read().decode("utf-8", "replace")
        sys.exit("error: HTTP %s from %s/sql\n%s" % (exc.code, SURREAL_URL, body))
    if not isinstance(payload, list) or not payload:
        sys.exit("error: unexpected response: %r" % (payload,))
    first = payload[0]
    if first.get("status") != "OK":
        sys.exit("error: query failed: %s\nquery: %s" % (first.get("result"), query))
    return first.get("result")


def esc(value):
    """Escape a Python string as a SurrealQL double-quoted string literal."""
    out = []
    for ch in value:
        if ch == '"':
            out.append('\\"')
        elif ch == "\\":
            out.append("\\\\")
        elif ch == "\n":
            out.append("\\n")
        elif ch == "\r":
            out.append("\\r")
        elif ch == "\t":
            out.append("\\t")
        elif ord(ch) < 0x20:
            out.append("\\u%04x" % ord(ch))
        else:
            out.append(ch)
    return '"' + "".join(out) + '"'


def val(value):
    if value is None:
        return "NONE"
    if value is True:
        return "true"
    if value is False:
        return "false"
    if isinstance(value, (int, float)):
        return repr(value)
    if isinstance(value, str):
        return esc(value)
    raise TypeError("unsupported value: %r" % (value,))


def record_id(raw):
    """Normalize a record id from a query result to embeddable `table:id` form."""
    if isinstance(raw, str):
        return raw
    if isinstance(raw, dict):
        if "tb" in raw and "id" in raw:
            inner = raw["id"]
            inner = inner.get("String", inner) if isinstance(inner, dict) else inner
            return "%s:%s" % (raw["tb"], inner)
        if "id" in raw:
            return record_id(raw["id"])
    raise TypeError("cannot use as record id: %r" % (raw,))


def first_id(rows):
    if not rows:
        return None
    row = rows[0]
    return record_id(row.get("id", row) if isinstance(row, dict) else row)


UNUSABLE_PASSWORD_HASH = "UNUSABLE_SEED_ACCOUNT"


def educator_names():
    """name -> id pairs for the educator accounts used by the seed files."""
    path = os.path.join(DATA_DIR, "educator_ids.json")
    names = {}
    if not os.path.exists(path):
        return names
    with open(path, encoding="utf-8") as fh:
        for row in json.load(fh):
            names[row["id"]] = row["name"]
    return names


def ensure_educator(rid, display_name=None):
    """Create the educator account for `rid` when it does not exist yet.

    Returns True when a new account was created.
    """
    rows = sql("SELECT VALUE id FROM users WHERE id = %s LIMIT 1" % esc(rid))
    if rows:
        return False
    name = display_name or ("Educator %s" % rid)
    sql(
        "UPSERT %s CONTENT { display_name: %s, password_hash: %s, role: 'educator',"
        " created_at: time::now(), updated_at: time::now() }"
        % (rid, esc(name), esc(UNUSABLE_PASSWORD_HASH))
    )
    print("  + educator account %s created (%s)" % (rid, name), flush=True)
    return True


def record_ids_of(rows):
    """Normalize a list of query rows into "table:id" strings."""
    out = []
    for row in rows or []:
        try:
            out.append(record_id(row))
        except TypeError:
            continue
    return out


def import_tracks(data_dir):
    """Create or update tracks, matched by slug (never duplicates)."""
    path = os.path.join(data_dir, "tracks.json")
    if not os.path.exists(path):
        sys.exit("error: tracks.json not found at %s" % path)
    with open(path, encoding="utf-8") as fh:
        tracks = json.load(fh)
    created, updated = 0, 0
    for track in tracks:
        rows = sql("SELECT VALUE id FROM tracks WHERE slug = %s LIMIT 1" % esc(track["slug"]))
        if rows:
            track_id = record_id(rows[0])
            sql(
                "UPDATE %s SET name = %s, description = %s, icon = %s, image_url = %s,"
                " sort_order = %d, deleted = false, updated_at = time::now()"
                % (
                    esc(track_id),
                    esc(track["name"]),
                    esc(track["description"]),
                    val(track.get("icon")),
                    val(track.get("image_url")),
                    int(track.get("sort_order", 0)),
                )
            )
            updated += 1
            print("  = track %s updated" % track["slug"], flush=True)
            continue
        sql(
            "CREATE tracks CONTENT { name: %s, slug: %s, description: %s,"
            " icon: %s, image_url: %s, sort_order: %d,"
            " created_at: time::now(), updated_at: time::now(), deleted: false }"
            % (
                esc(track["name"]),
                esc(track["slug"]),
                esc(track["description"]),
                val(track.get("icon")),
                val(track.get("image_url")),
                int(track.get("sort_order", 0)),
            )
        )
        created += 1
        print("  + track %s created" % track["slug"], flush=True)
    print("tracks: %d created, %d updated" % (created, updated))


def import_courses(data_dir):
    """Create or update courses, modules and lessons without duplicating.

    * course  -> keyed by slug
    * module  -> keyed by (course, title)
    * lesson  -> keyed by (module, title)

    Modules and lessons that disappeared from the seed files are soft-deleted
    so existing progress rows keep pointing at valid records.
    """
    directory = os.path.join(data_dir, "courses")
    if not os.path.isdir(directory):
        sys.exit("error: courses directory not found at %s" % directory)
    files = sorted(f for f in os.listdir(directory) if f.endswith(".json"))
    educator_display_names = educator_names()
    created, updated = 0, 0

    for filename in files:
        with open(os.path.join(directory, filename), encoding="utf-8") as fh:
            course = json.load(fh)

        track_rows = sql(
            "SELECT VALUE id FROM tracks WHERE slug = %s LIMIT 1" % esc(course["track_slug"])
        )
        if not track_rows:
            sys.exit(
                "error: Track not found for slug %s (import tracks first)"
                % course["track_slug"]
            )
        track_id = record_id(track_rows[0])

        educator_id = course["educator_id"]
        ensure_educator(
            educator_id, educator_display_names.get(educator_id)
        )

        course_rows = sql(
            "SELECT VALUE id FROM courses WHERE slug = %s LIMIT 1" % esc(course["slug"])
        )
        if course_rows:
            course_id = record_id(course_rows[0])
            sql(
                "UPDATE %s SET title = %s, description = %s, short_description = %s,"
                " track = %s, educator = %s, level = %s, status = %s, language = %s,"
                " thumbnail_url = %s, video_url = %s, deleted = false, updated_at = time::now()"
                % (
                    esc(course_id),
                    esc(course["title"]),
                    esc(course["description"]),
                    esc(course["short_description"]),
                    esc(track_id),
                    esc(educator_id),
                    esc(course["level"]),
                    esc("published"),
                    esc(course.get("language") or "en"),
                    val(course.get("thumbnail_url")),
                    val(course.get("video_url")),
                )
            )
            updated += 1
            print("  = course %s updated" % course["slug"], flush=True)
        else:
            course_rows = sql(
                "CREATE courses CONTENT { title: %s, slug: %s, description: %s,"
                " short_description: %s, track: %s, educator: %s, level: %s,"
                " status: %s, language: %s, thumbnail_url: %s, video_url: %s,"
                " duration_minutes: 0, lesson_count: 0, enrollment_count: 0,"
                " created_at: time::now(), updated_at: time::now(), deleted: false }"
                % (
                    esc(course["title"]),
                    esc(course["slug"]),
                    esc(course["description"]),
                    esc(course["short_description"]),
                    esc(track_id),
                    esc(educator_id),
                    esc(course["level"]),
                    esc("published"),
                    esc(course.get("language") or "en"),
                    val(course.get("thumbnail_url")),
                    val(course.get("video_url")),
                )
            )
            course_id = first_id(course_rows)
            if course_id is None:
                sys.exit("error: Failed to create course %s" % course["slug"])
            created += 1
            print("  + course %s created" % course["slug"], flush=True)

        kept_module_ids = []
        for module in course.get("modules", []):
            module_rows = sql(
                "SELECT VALUE id FROM modules WHERE course = %s AND title = %s LIMIT 1"
                % (esc(course_id), esc(module["title"]))
            )
            if module_rows:
                module_id = record_id(module_rows[0])
                sql(
                    "UPDATE %s SET description = %s, sort_order = %s, deleted = false,"
                    " updated_at = time::now()"
                    % (
                        esc(module_id),
                        val(module.get("description")),
                        val(module.get("sort_order", 0)),
                    )
                )
            else:
                module_rows = sql(
                    "CREATE modules CONTENT { title: %s, course: %s,"
                    " description: %s, sort_order: %s,"
                    " created_at: time::now(), updated_at: time::now(),"
                    " deleted: false }"
                    % (
                        esc(module["title"]),
                        esc(course_id),
                        val(module.get("description")),
                        val(module.get("sort_order", 0)),
                    )
                )
                module_id = first_id(module_rows)
                if module_id is None:
                    sys.exit("error: Failed to create module in %s" % course["slug"])
            kept_module_ids.append(module_id)

            kept_lesson_ids = []
            for lesson in module.get("lessons", []):
                lesson_rows = sql(
                    "SELECT VALUE id FROM lessons WHERE module = %s AND title = %s LIMIT 1"
                    % (esc(module_id), esc(lesson["title"]))
                )
                if lesson_rows:
                    lesson_id = record_id(lesson_rows[0])
                    sql(
                        "UPDATE %s SET content_type = %s, content = %s, video_url = %s,"
                        " video_duration_seconds = %s, audio_url = %s, pdf_url = %s,"
                        " external_url = %s, thumbnail_url = %s, duration_minutes = %s,"
                        " sort_order = %s, is_preview = %s, deleted = false,"
                        " updated_at = time::now()"
                        % (
                            esc(lesson_id),
                            esc(lesson["content_type"]),
                            esc(lesson["content"]),
                            val(lesson.get("video_url")),
                            val(lesson.get("video_duration_seconds")),
                            val(lesson.get("audio_url")),
                            val(lesson.get("pdf_url")),
                            val(lesson.get("external_url")),
                            val(lesson.get("thumbnail_url")),
                            val(lesson.get("duration_minutes", 5)),
                            val(lesson.get("sort_order", 0)),
                            val(lesson.get("is_preview", False)),
                        )
                    )
                else:
                    sql(
                        "CREATE lessons CONTENT { title: %s, module: %s,"
                        " content_type: %s, content: %s, video_url: %s,"
                        " video_duration_seconds: %s, audio_url: %s, pdf_url: %s,"
                        " external_url: %s, thumbnail_url: %s,"
                        " duration_minutes: %s, sort_order: %s, is_preview: %s,"
                        " created_at: time::now(), updated_at: time::now(),"
                        " deleted: false }"
                        % (
                            esc(lesson["title"]),
                            esc(module_id),
                            esc(lesson["content_type"]),
                            esc(lesson["content"]),
                            val(lesson.get("video_url")),
                            val(lesson.get("video_duration_seconds")),
                            val(lesson.get("audio_url")),
                            val(lesson.get("pdf_url")),
                            val(lesson.get("external_url")),
                            val(lesson.get("thumbnail_url")),
                            val(lesson.get("duration_minutes", 5)),
                            val(lesson.get("sort_order", 0)),
                            val(lesson.get("is_preview", False)),
                        )
                    )
                    lesson_rows = sql(
                        "SELECT VALUE id FROM lessons WHERE module = %s AND title = %s LIMIT 1"
                        % (esc(module_id), esc(lesson["title"]))
                    )
                    if not lesson_rows:
                        sys.exit("error: Failed to create lesson in %s" % course["slug"])
                    lesson_id = record_id(lesson_rows[0])
                kept_lesson_ids.append(lesson_id)

            # Soft-delete lessons that are no longer part of the seed.
            stale = sql(
                "SELECT VALUE id FROM lessons WHERE module = %s AND deleted = false"
                % esc(module_id)
            )
            for stale_id in record_ids_of(stale):
                if stale_id not in kept_lesson_ids:
                    sql(
                        "UPDATE %s SET deleted = true, updated_at = time::now()"
                        % esc(stale_id)
                    )

        # Soft-delete modules that are no longer part of the seed.
        stale = sql(
            "SELECT VALUE id FROM modules WHERE course = %s AND deleted = false" % esc(course_id)
        )
        for stale_id in record_ids_of(stale):
            if stale_id not in kept_module_ids:
                sql(
                    "UPDATE %s SET deleted = true, updated_at = time::now()" % esc(stale_id)
                )

        lesson_total = len(
            sql(
                "SELECT VALUE id FROM lessons WHERE module IN"
                " (SELECT VALUE id FROM modules WHERE course = %s AND deleted = false)"
                " AND deleted = false" % esc(course_id)
            )
        )
        sql(
            "UPDATE %s SET lesson_count = %d, updated_at = time::now()"
            % (esc(course_id), lesson_total)
        )
    print("courses: %d created, %d updated" % (created, updated))


def sync_relation(table, parent_id, child_id, assignments):
    """Create or update one relation edge, keyed by its (in, out) pair."""
    rows = sql(
        "SELECT VALUE id FROM %s WHERE in = %s AND out = %s LIMIT 1"
        % (table, esc(parent_id), esc(child_id))
    )
    if rows:
        edge_id = record_id(rows[0])
        if assignments:
            sql(
                "UPDATE %s SET %s"
                % (
                    esc(edge_id),
                    ", ".join("%s = %s" % (k, v) for k, v in assignments),
                )
            )
        return
    statement = "RELATE %s -> %s -> %s" % (esc(parent_id), table, esc(child_id))
    if assignments:
        statement += " SET %s" % ", ".join("%s = %s" % (k, v) for k, v in assignments)
    sql(statement)


def import_roadmaps(data_dir):
    """Create or update roadmaps and their course links, keyed by slug."""
    directory = os.path.join(data_dir, "roadmaps")
    if not os.path.isdir(directory):
        sys.exit("error: roadmaps directory not found at %s" % directory)
    files = sorted(f for f in os.listdir(directory) if f.endswith(".json"))
    created, updated = 0, 0

    for filename in files:
        with open(os.path.join(directory, filename), encoding="utf-8") as fh:
            roadmap = json.load(fh)

        track_id = None
        if roadmap.get("track_slug"):
            track_rows = sql(
                "SELECT VALUE id FROM tracks WHERE slug = %s LIMIT 1"
                % esc(roadmap["track_slug"])
            )
            if track_rows:
                track_id = record_id(track_rows[0])
            else:
                print(
                    "  ! track %s not found, leaving empty" % roadmap["track_slug"],
                    flush=True,
                )
        created_by = resolve_created_by(roadmap.get("created_by"), roadmap["slug"])
        status = roadmap.get("status") or "draft"

        roadmap_rows = sql(
            "SELECT VALUE id FROM roadmaps WHERE slug = %s LIMIT 1" % esc(roadmap["slug"])
        )
        if roadmap_rows:
            roadmap_id = record_id(roadmap_rows[0])
            sql(
                "UPDATE %s SET title = %s, description = %s, image_url = %s, track = %s,"
                " difficulty = %s, estimated_weeks = %d, status = %s, created_by = %s,"
                " deleted = false, updated_at = time::now()"
                % (
                    esc(roadmap_id),
                    esc(roadmap["title"]),
                    esc(roadmap["description"]),
                    val(roadmap.get("image_url")),
                    esc(track_id) if track_id else "NONE",
                    esc(roadmap["difficulty"]),
                    int(roadmap["estimated_weeks"]),
                    esc(status),
                    esc(created_by),
                )
            )
            updated += 1
            print("  = roadmap %s updated" % roadmap["slug"], flush=True)
        else:
            roadmap_rows = sql(
                "CREATE roadmaps CONTENT { title: %s, slug: %s, description: %s,"
                " image_url: %s, track: %s, difficulty: %s, estimated_weeks: %d,"
                " status: %s, created_by: %s,"
                " created_at: time::now(), updated_at: time::now(), deleted: false }"
                % (
                    esc(roadmap["title"]),
                    esc(roadmap["slug"]),
                    esc(roadmap["description"]),
                    val(roadmap.get("image_url")),
                    esc(track_id) if track_id else "NONE",
                    esc(roadmap["difficulty"]),
                    int(roadmap["estimated_weeks"]),
                    esc(status),
                    esc(created_by),
                )
            )
            roadmap_id = first_id(roadmap_rows)
            if roadmap_id is None:
                sys.exit("error: Failed to create roadmap %s" % roadmap["slug"])
            created += 1
            print("  + roadmap %s created" % roadmap["slug"], flush=True)

        kept_course_ids = []
        for course in roadmap.get("courses", []):
            course_rows = sql(
                "SELECT VALUE id FROM courses WHERE slug = %s LIMIT 1"
                % esc(course["course_slug"])
            )
            if not course_rows:
                sys.exit(
                    "error: Course not found for roadmap: %s" % course["course_slug"]
                )
            course_id = record_id(course_rows[0])
            sync_relation(
                "roadmap_courses",
                roadmap_id,
                course_id,
                [
                    ("sort_order", val(course.get("sort_order", 0))),
                    ("is_required", val(course.get("is_required", True))),
                    ("note", val(course.get("note"))),
                ],
            )
            kept_course_ids.append(course_id)

        # Remove links for courses that dropped out of the roadmap.
        edges = sql("SELECT VALUE id FROM roadmap_courses WHERE in = %s" % esc(roadmap_id))
        for edge_id in record_ids_of(edges):
            out_rows = sql(
                "SELECT VALUE out FROM roadmap_courses WHERE id = %s LIMIT 1" % esc(edge_id)
            )
            if not out_rows:
                continue
            out_id = record_id(out_rows[0])
            if out_id not in kept_course_ids:
                sql("DELETE %s" % esc(edge_id))
    print("roadmaps: %d created, %d updated" % (created, updated))


def import_frameworks(data_dir):
    """Create or update frameworks, milestones and their course links."""
    directory = os.path.join(data_dir, "frameworks")
    if not os.path.isdir(directory):
        sys.exit("error: frameworks directory not found at %s" % directory)
    files = sorted(f for f in os.listdir(directory) if f.endswith(".json"))
    created, updated = 0, 0

    for filename in files:
        with open(os.path.join(directory, filename), encoding="utf-8") as fh:
            framework = json.load(fh)

        track_id = None
        if framework.get("track_slug"):
            track_rows = sql(
                "SELECT VALUE id FROM tracks WHERE slug = %s LIMIT 1"
                % esc(framework["track_slug"])
            )
            if track_rows:
                track_id = record_id(track_rows[0])
            else:
                print(
                    "  ! track %s not found, leaving empty" % framework["track_slug"],
                    flush=True,
                )
        created_by = resolve_created_by(framework.get("created_by"), framework["slug"])
        status = framework.get("status") or "draft"

        framework_rows = sql(
            "SELECT VALUE id FROM frameworks WHERE slug = %s LIMIT 1"
            % esc(framework["slug"])
        )
        if framework_rows:
            framework_id = record_id(framework_rows[0])
            sql(
                "UPDATE %s SET title = %s, description = %s, image_url = %s, track = %s,"
                " status = %s, created_by = %s, deleted = false, updated_at = time::now()"
                % (
                    esc(framework_id),
                    esc(framework["title"]),
                    esc(framework["description"]),
                    val(framework.get("image_url")),
                    esc(track_id) if track_id else "NONE",
                    esc(status),
                    esc(created_by),
                )
            )
            updated += 1
            print("  = framework %s updated" % framework["slug"], flush=True)
        else:
            framework_rows = sql(
                "CREATE frameworks CONTENT { title: %s, slug: %s, description: %s,"
                " image_url: %s, track: %s, status: %s, created_by: %s,"
                " created_at: time::now(), updated_at: time::now(), deleted: false }"
                % (
                    esc(framework["title"]),
                    esc(framework["slug"]),
                    esc(framework["description"]),
                    val(framework.get("image_url")),
                    esc(track_id) if track_id else "NONE",
                    esc(status),
                    esc(created_by),
                )
            )
            framework_id = first_id(framework_rows)
            if framework_id is None:
                sys.exit("error: Failed to create framework %s" % framework["slug"])
            created += 1
            print("  + framework %s created" % framework["slug"], flush=True)

        for milestone in framework.get("milestones", []):
            milestone_rows = sql(
                "SELECT VALUE id FROM milestones WHERE framework = %s AND title = %s LIMIT 1"
                % (esc(framework_id), esc(milestone["title"]))
            )
            if milestone_rows:
                milestone_id = record_id(milestone_rows[0])
                sql(
                    "UPDATE %s SET description = %s, sort_order = %s"
                    % (
                        esc(milestone_id),
                        val(milestone.get("description")),
                        val(milestone["sort_order"]),
                    )
                )
            else:
                milestone_rows = sql(
                    "CREATE milestones CONTENT { framework: %s, title: %s,"
                    " description: %s, sort_order: %s }"
                    % (
                        esc(framework_id),
                        esc(milestone["title"]),
                        val(milestone.get("description")),
                        val(milestone["sort_order"]),
                    )
                )
                milestone_id = first_id(milestone_rows)
                if milestone_id is None:
                    sys.exit(
                        "error: Failed to create milestone in %s" % framework["slug"]
                    )

            kept_course_ids = []
            for course in milestone.get("courses", []):
                course_rows = sql(
                    "SELECT VALUE id FROM courses WHERE slug = %s LIMIT 1"
                    % esc(course["course_slug"])
                )
                if not course_rows:
                    sys.exit(
                        "error: Course not found for milestone: %s"
                        % course["course_slug"]
                    )
                course_id = record_id(course_rows[0])
                sync_relation(
                    "milestone_courses",
                    milestone_id,
                    course_id,
                    [("is_required", val(course.get("is_required", True)))],
                )
                kept_course_ids.append(course_id)

            edges = sql(
                "SELECT VALUE id FROM milestone_courses WHERE in = %s" % esc(milestone_id)
            )
            for edge_id in record_ids_of(edges):
                out_rows = sql(
                    "SELECT VALUE out FROM milestone_courses WHERE id = %s LIMIT 1"
                    % esc(edge_id)
                )
                if not out_rows:
                    continue
                out_id = record_id(out_rows[0])
                if out_id not in kept_course_ids:
                    sql("DELETE %s" % esc(edge_id))
    print("frameworks: %d created, %d updated" % (created, updated))


def main(argv):
    args = set(argv[1:])
    allowed = {"--tracks", "--courses", "--roadmaps", "--frameworks", "--all"}
    if not args or args - allowed:
        print(
            "Usage: python3 scripts/import_db.py"
            " --tracks|--courses|--roadmaps|--frameworks|--all",
            flush=True,
        )
        return 1
    if "--all" in args:
        args = {"--tracks", "--courses", "--roadmaps", "--frameworks"}
    if "--tracks" in args:
        import_tracks(DATA_DIR)
    if "--courses" in args:
        import_courses(DATA_DIR)
    if "--roadmaps" in args:
        import_roadmaps(DATA_DIR)
    if "--frameworks" in args:
        import_frameworks(DATA_DIR)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
