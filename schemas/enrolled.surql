DEFINE TABLE IF NOT EXISTS enrolled SCHEMAFULL TYPE RELATION FROM users TO courses;

DEFINE FIELD IF NOT EXISTS enrolled_at ON enrolled TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS completed_at ON enrolled TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS progress_percent ON enrolled TYPE float DEFAULT 0;
DEFINE FIELD IF NOT EXISTS last_accessed_at ON enrolled TYPE option<datetime>;

DEFINE INDEX IF NOT EXISTS enrolled_unique_idx ON enrolled FIELDS in, out UNIQUE;
