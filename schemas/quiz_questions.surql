DEFINE TABLE IF NOT EXISTS quiz_questions SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS quiz ON quiz_questions TYPE record<quizzes>;
DEFINE FIELD IF NOT EXISTS question_text ON quiz_questions TYPE string;
DEFINE FIELD IF NOT EXISTS question_type ON quiz_questions TYPE string 
    ASSERT $value IN ["multiple_choice", "true_false"];
DEFINE FIELD IF NOT EXISTS options ON quiz_questions TYPE option<array<string>>;
DEFINE FIELD IF NOT EXISTS correct_answer ON quiz_questions TYPE string;
DEFINE FIELD IF NOT EXISTS explanation ON quiz_questions TYPE option<string>;
DEFINE FIELD IF NOT EXISTS sort_order ON quiz_questions TYPE int DEFAULT 0;
