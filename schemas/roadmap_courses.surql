DEFINE TABLE IF NOT EXISTS roadmap_courses SCHEMAFULL TYPE RELATION FROM roadmaps TO courses;

DEFINE FIELD IF NOT EXISTS sort_order ON roadmap_courses TYPE int;
DEFINE FIELD IF NOT EXISTS is_required ON roadmap_courses TYPE bool DEFAULT true;
DEFINE FIELD IF NOT EXISTS note ON roadmap_courses TYPE option<string>;

DEFINE INDEX IF NOT EXISTS roadmap_courses_unique_idx ON roadmap_courses FIELDS in, out UNIQUE;
