DEFINE TABLE IF NOT EXISTS tracks SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON tracks TYPE string;
DEFINE FIELD IF NOT EXISTS slug ON tracks TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD IF NOT EXISTS description ON tracks TYPE string;
DEFINE FIELD IF NOT EXISTS icon ON tracks TYPE option<string>;
DEFINE FIELD IF NOT EXISTS image_url ON tracks TYPE option<string>;
DEFINE FIELD IF NOT EXISTS sort_order ON tracks TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS created_at ON tracks TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON tracks TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON tracks TYPE bool DEFAULT false;

DEFINE INDEX IF NOT EXISTS track_slug_idx ON tracks FIELDS slug UNIQUE;
