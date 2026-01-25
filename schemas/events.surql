DEFINE TABLE IF NOT EXISTS events SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON events TYPE string;
DEFINE FIELD IF NOT EXISTS description ON events TYPE string;
DEFINE FIELD IF NOT EXISTS category ON events TYPE string;
DEFINE FIELD IF NOT EXISTS date ON events TYPE datetime;
DEFINE FIELD IF NOT EXISTS mosque ON events TYPE record<mosques>;
DEFINE FIELD IF NOT EXISTS speaker ON events TYPE option<string>;
