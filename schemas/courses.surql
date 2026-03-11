DEFINE TABLE IF NOT EXISTS courses SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON courses TYPE string;
DEFINE FIELD IF NOT EXISTS slug ON courses TYPE string;
DEFINE FIELD IF NOT EXISTS description ON courses TYPE string;
DEFINE FIELD IF NOT EXISTS short_description ON courses TYPE string;
DEFINE FIELD IF NOT EXISTS track ON courses TYPE record<tracks>;
DEFINE FIELD IF NOT EXISTS educator ON courses TYPE record<users>;
DEFINE FIELD IF NOT EXISTS level ON courses TYPE string 
    ASSERT $value IN ["beginner", "intermediate", "advanced"];
DEFINE FIELD IF NOT EXISTS status ON courses TYPE string 
    ASSERT $value IN ["draft", "review", "published", "archived"]
    DEFAULT "draft";
DEFINE FIELD IF NOT EXISTS language ON courses TYPE string DEFAULT "en";
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
