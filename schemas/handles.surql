-- Graph edge: user administers/handles a mosque
DEFINE TABLE IF NOT EXISTS handles SCHEMAFULL TYPE RELATION IN users OUT mosques;

-- When was this permission granted
DEFINE FIELD IF NOT EXISTS granted_at ON handles TYPE datetime DEFAULT time::now();

-- Who granted it (optional audit trail)
DEFINE FIELD IF NOT EXISTS granted_by ON handles TYPE option<record<users>>;

-- Prevent duplicate edges
DEFINE INDEX IF NOT EXISTS idx_handles_unique ON handles FIELDS in, out UNIQUE;
