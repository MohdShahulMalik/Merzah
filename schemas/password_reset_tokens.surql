DEFINE TABLE IF NOT EXISTS password_reset_tokens SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON password_reset_tokens TYPE record<users> ASSERT $value != NONE;
-- SHA256 hex of the raw reset token. Raw token is only returned once to the requester.
DEFINE FIELD IF NOT EXISTS token_hash ON password_reset_tokens TYPE string ASSERT string::len($value) == 64;
DEFINE FIELD IF NOT EXISTS created_at ON password_reset_tokens TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS expires_at ON password_reset_tokens TYPE datetime;

DEFINE INDEX IF NOT EXISTS idx_reset_token_hash ON TABLE password_reset_tokens COLUMNS token_hash UNIQUE;
