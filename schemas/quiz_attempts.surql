DEFINE TABLE IF NOT EXISTS quiz_attempts SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON quiz_attempts TYPE record<users>;
DEFINE FIELD IF NOT EXISTS quiz ON quiz_attempts TYPE record<quizzes>;
DEFINE FIELD IF NOT EXISTS answers ON quiz_attempts TYPE array;
DEFINE FIELD IF NOT EXISTS score ON quiz_attempts TYPE float;
DEFINE FIELD IF NOT EXISTS passed ON quiz_attempts TYPE bool;
DEFINE FIELD IF NOT EXISTS attempted_at ON quiz_attempts TYPE datetime DEFAULT time::now();
