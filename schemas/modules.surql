DEFINE TABLE IF NOT EXISTS modules SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON modules TYPE string;
DEFINE FIELD IF NOT EXISTS course ON modules TYPE record<courses>;
DEFINE FIELD IF NOT EXISTS description ON modules TYPE option<string>;
DEFINE FIELD IF NOT EXISTS sort_order ON modules TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS created_at ON modules TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON modules TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON modules TYPE bool DEFAULT false;

DEFINE INDEX IF NOT EXISTS module_course_idx ON modules FIELDS course;
