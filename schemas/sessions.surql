DEFINE TABLE IF NOT EXISTS sessions SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON sessions TYPE record<users> ASSERT $value != NONE;
-- Stores SHA256 hex of the raw session token. Never store raw tokens.
DEFINE FIELD IF NOT EXISTS session_token ON sessions TYPE string ASSERT string::len($value) == 64;
DEFINE FIELD IF NOT EXISTS created_at ON sessions TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS expires_at ON sessions TYPE datetime;

-- Unique Session Tokens (hashed)
DEFINE INDEX IF NOT EXISTS idx_session_token ON TABLE sessions COLUMNS session_token UNIQUE;
