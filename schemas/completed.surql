DEFINE TABLE IF NOT EXISTS completed SCHEMAFULL TYPE RELATION FROM users TO lessons;

DEFINE FIELD IF NOT EXISTS completed_at ON completed TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS time_spent_seconds ON completed TYPE option<int>;

DEFINE INDEX IF NOT EXISTS completed_unique_idx ON completed FIELDS in, out UNIQUE;
