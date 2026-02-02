-- Graph edge users attending events
DEFINE TABLE IF NOT EXISTS attending SCHEMAFULL TYPE RELATION IN users OUT events;

DEFINE FIELD IF NOT EXISTS created_at ON attending TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS idx_handles_unique ON handles FIELDS in, out UNIQUE;
