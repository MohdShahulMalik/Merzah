DEFINE TABLE IF NOT EXISTS earned SCHEMAFULL TYPE RELATION FROM users TO achievements;

DEFINE FIELD IF NOT EXISTS earned_at ON earned TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS earned_unique_idx ON earned FIELDS in, out UNIQUE;
