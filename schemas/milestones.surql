DEFINE TABLE IF NOT EXISTS milestones SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS framework ON milestones TYPE record<frameworks>;
DEFINE FIELD IF NOT EXISTS title ON milestones TYPE string;
DEFINE FIELD IF NOT EXISTS description ON milestones TYPE option<string>;
DEFINE FIELD IF NOT EXISTS sort_order ON milestones TYPE int;
