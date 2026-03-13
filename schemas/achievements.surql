DEFINE TABLE IF NOT EXISTS achievements SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON achievements TYPE string;
DEFINE FIELD IF NOT EXISTS slug ON achievements TYPE string UNIQUE;
DEFINE FIELD IF NOT EXISTS description ON achievements TYPE string;
DEFINE FIELD IF NOT EXISTS icon ON achievements TYPE string;
DEFINE FIELD IF NOT EXISTS category ON achievements TYPE string 
    ASSERT $value IN ["learning", "streak", "social", "milestone"];
DEFINE FIELD IF NOT EXISTS requirement_type ON achievements TYPE string;
DEFINE FIELD IF NOT EXISTS requirement_value ON achievements TYPE int;
DEFINE FIELD IF NOT EXISTS points ON achievements TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS created_at ON achievements TYPE datetime DEFAULT time::now();
