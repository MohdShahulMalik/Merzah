DEFINE TABLE IF NOT EXISTS lessons SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON lessons TYPE string;
DEFINE FIELD IF NOT EXISTS module ON lessons TYPE record<modules>;
DEFINE FIELD IF NOT EXISTS content_type ON lessons TYPE string 
    ASSERT $value IN ["text", "video", "audio", "pdf", "external_link", "mixed"];
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
