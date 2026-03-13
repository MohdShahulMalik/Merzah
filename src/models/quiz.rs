use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum QuestionType {
    MultipleChoice,
    TrueFalse,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    pub id: RecordId,
    pub lesson: RecordId,
    pub title: String,
    pub passing_score: f32,
    pub max_attempts: i32,
    pub created_at: Datetime,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct QuizQuestion {
    pub id: RecordId,
    pub quiz: RecordId,
    pub question_text: String,
    pub question_type: QuestionType,
    pub options: Option<Vec<String>>,
    pub correct_answer: String,
    pub explanation: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuizOnClient {
    pub id: String,
    pub lesson_id: String,
    pub title: String,
    pub passing_score: f32,
    pub max_attempts: i32,
    pub questions: Vec<QuizQuestionOnClient>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuizQuestionOnClient {
    pub id: String,
    pub question_text: String,
    pub question_type: QuestionType,
    pub options: Option<Vec<String>>,
    pub sort_order: i32,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct QuizAttempt {
    pub id: RecordId,
    pub user: RecordId,
    pub quiz: RecordId,
    pub answers: Vec<QuizAnswer>,
    pub score: f32,
    pub passed: bool,
    pub attempted_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuizAnswer {
    pub question_id: String,
    pub answer: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuizSubmission {
    pub quiz_id: String,
    pub answers: Vec<QuizAnswer>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuizSubmissionResult {
    pub score: f32,
    pub passed: bool,
    pub correct_count: i32,
    pub total_questions: i32,
}
