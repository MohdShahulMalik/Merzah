DEFINE TABLE IF NOT EXISTS milestone_courses SCHEMAFULL TYPE RELATION FROM milestones TO courses;

DEFINE FIELD IF NOT EXISTS is_required ON milestone_courses TYPE bool DEFAULT true;

DEFINE INDEX IF NOT EXISTS milestone_courses_unique_idx ON milestone_courses FIELDS in, out UNIQUE;
