DEFINE TABLE IF NOT EXISTS quizzes SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS lesson ON quizzes TYPE record<lessons>;
DEFINE FIELD IF NOT EXISTS title ON quizzes TYPE string;
DEFINE FIELD IF NOT EXISTS passing_score ON quizzes TYPE float DEFAULT 0.7;
DEFINE FIELD IF NOT EXISTS max_attempts ON quizzes TYPE int DEFAULT 3;
DEFINE FIELD IF NOT EXISTS created_at ON quizzes TYPE datetime DEFAULT time::now();
