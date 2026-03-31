DEFINE TABLE IF NOT EXISTS frameworks SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON frameworks TYPE string;
DEFINE FIELD IF NOT EXISTS slug ON frameworks TYPE string;
DEFINE FIELD IF NOT EXISTS description ON frameworks TYPE string;
DEFINE FIELD IF NOT EXISTS image_url ON frameworks TYPE option<string>;
DEFINE FIELD IF NOT EXISTS track ON frameworks TYPE option<record<tracks>>;
DEFINE FIELD IF NOT EXISTS status ON frameworks TYPE string DEFAULT "draft";
DEFINE FIELD IF NOT EXISTS created_by ON frameworks TYPE record<users>;
DEFINE FIELD IF NOT EXISTS created_at ON frameworks TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON frameworks TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON frameworks TYPE bool DEFAULT false;

DEFINE INDEX IF NOT EXISTS framework_slug_idx ON frameworks FIELDS slug UNIQUE;
