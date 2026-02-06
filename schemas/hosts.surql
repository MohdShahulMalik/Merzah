-- Graph edge: mosques hosts events
DEFINE TABLE IF NOT EXISTS hosts SCHEMAFULL TYPE RELATION IN mosques OUT events;

DEFINE FIELD IF NOT EXISTS created_at ON hosts TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS created_by ON hosts TYPE record<users> ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS updated_at ON hosts TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS idx_handles_unique ON handles FIELDS in, out UNIQUE;
