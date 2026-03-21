DEFINE TABLE IF NOT EXISTS roadmaps SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON roadmaps TYPE string;
DEFINE FIELD IF NOT EXISTS slug ON roadmaps TYPE string;
DEFINE FIELD IF NOT EXISTS description ON roadmaps TYPE string;
DEFINE FIELD IF NOT EXISTS image_url ON roadmaps TYPE option<string>;
DEFINE FIELD IF NOT EXISTS track ON roadmaps TYPE option<record<tracks>>;
DEFINE FIELD IF NOT EXISTS difficulty ON roadmaps TYPE string 
    ASSERT $value IN ["beginner", "intermediate", "advanced", "all"];
DEFINE FIELD IF NOT EXISTS estimated_weeks ON roadmaps TYPE int;
DEFINE FIELD IF NOT EXISTS status ON roadmaps TYPE string 
    ASSERT $value IN ["draft", "published", "archived"] DEFAULT "draft";
DEFINE FIELD IF NOT EXISTS created_by ON roadmaps TYPE record<users>;
DEFINE FIELD IF NOT EXISTS created_at ON roadmaps TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON roadmaps TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON roadmaps TYPE bool DEFAULT false;

DEFINE INDEX IF NOT EXISTS roadmap_slug_idx ON roadmaps FIELDS slug UNIQUE;
