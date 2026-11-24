DEFINE TABLE IF NOT EXISTS achievements SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON achievements TYPE string;
DEFINE FIELD IF NOT EXISTS slug ON achievements TYPE string;
DEFINE FIELD IF NOT EXISTS description ON achievements TYPE string;
DEFINE FIELD IF NOT EXISTS icon ON achievements TYPE string;
DEFINE FIELD IF NOT EXISTS category ON achievements TYPE string
    ASSERT $value IN ["learning", "streak", "social", "milestone"];
DEFINE FIELD IF NOT EXISTS requirement_type ON achievements TYPE string;
DEFINE FIELD IF NOT EXISTS requirement_value ON achievements TYPE int;
DEFINE FIELD IF NOT EXISTS points ON achievements TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS created_at ON achievements TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS achievement_slug_idx ON achievements FIELDS slug UNIQUE;

-- Graph edge users attending events
DEFINE TABLE IF NOT EXISTS attending SCHEMAFULL TYPE RELATION IN users OUT events;

DEFINE FIELD IF NOT EXISTS created_at ON attending TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS idx_handles_unique ON handles FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS certificates SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON certificates TYPE record<users>;
DEFINE FIELD IF NOT EXISTS course ON certificates TYPE record<courses>;
DEFINE FIELD IF NOT EXISTS certificate_number ON certificates TYPE string;
DEFINE FIELD IF NOT EXISTS issued_at ON certificates TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS pdf_url ON certificates TYPE option<string>;

DEFINE INDEX IF NOT EXISTS certificate_number_idx ON certificates FIELDS certificate_number UNIQUE;
DEFINE INDEX IF NOT EXISTS cert_user_course_idx ON certificates FIELDS user, course UNIQUE;

DEFINE TABLE IF NOT EXISTS completed SCHEMAFULL TYPE RELATION FROM users TO lessons;

DEFINE FIELD IF NOT EXISTS completed_at ON completed TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS time_spent_seconds ON completed TYPE option<int>;

DEFINE INDEX IF NOT EXISTS completed_unique_idx ON completed FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS courses SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON courses TYPE string;
DEFINE FIELD IF NOT EXISTS slug ON courses TYPE string;
DEFINE FIELD IF NOT EXISTS description ON courses TYPE string;
DEFINE FIELD IF NOT EXISTS short_description ON courses TYPE string;
DEFINE FIELD IF NOT EXISTS track ON courses TYPE record<tracks>;
DEFINE FIELD IF NOT EXISTS educator ON courses TYPE record<users>;
DEFINE FIELD IF NOT EXISTS level ON courses TYPE string
    ASSERT $value IN ["beginner", "intermediate", "advanced"];
DEFINE FIELD IF NOT EXISTS status ON courses TYPE string
    ASSERT $value IN ["draft", "review", "published", "archived"]
    DEFAULT "draft";
DEFINE FIELD IF NOT EXISTS language ON courses TYPE string DEFAULT "en";
DEFINE FIELD IF NOT EXISTS thumbnail_url ON courses TYPE option<string>;
DEFINE FIELD IF NOT EXISTS video_url ON courses TYPE option<string>;
DEFINE FIELD IF NOT EXISTS duration_minutes ON courses TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS lesson_count ON courses TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS enrollment_count ON courses TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS created_at ON courses TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON courses TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON courses TYPE bool DEFAULT false;

DEFINE INDEX IF NOT EXISTS course_slug_idx ON courses FIELDS slug UNIQUE;
DEFINE INDEX IF NOT EXISTS course_track_idx ON courses FIELDS track;
DEFINE INDEX IF NOT EXISTS course_status_idx ON courses FIELDS status;

DEFINE TABLE IF NOT EXISTS earned SCHEMAFULL TYPE RELATION FROM users TO achievements;

DEFINE FIELD IF NOT EXISTS earned_at ON earned TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS earned_unique_idx ON earned FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS enrolled SCHEMAFULL TYPE RELATION FROM users TO courses;

DEFINE FIELD IF NOT EXISTS enrolled_at ON enrolled TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS completed_at ON enrolled TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS progress_percent ON enrolled TYPE float DEFAULT 0;
DEFINE FIELD IF NOT EXISTS last_accessed_at ON enrolled TYPE option<datetime>;

DEFINE INDEX IF NOT EXISTS enrolled_unique_idx ON enrolled FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS events SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON events TYPE string;
DEFINE FIELD IF NOT EXISTS description ON events TYPE string;
DEFINE FIELD IF NOT EXISTS category ON events TYPE string;
DEFINE FIELD IF NOT EXISTS date ON events TYPE datetime;
DEFINE FIELD IF NOT EXISTS speaker ON events TYPE option<string>;
DEFINE FIELD IF NOT EXISTS mosque ON events TYPE record<mosques>;

DEFINE FIELD IF NOT EXISTS recurrence_pattern ON events TYPE option<string>
    ASSERT
        ($value == NONE OR $value IN ["daily", "weekly", "biweekly", "monthly", "yearly", "weekdays", "weekends", "quaterly"])
        AND
        -- Logic: If pattern exists, end_date cannot be NONE
        ($value == NONE OR recurrence_end_date != NONE);

DEFINE FIELD IF NOT EXISTS recurrence_end_date ON events TYPE option<datetime>
    ASSERT
        -- Logic: If an end date is provided, a recurrence pattern MUST be set
        ($value == NONE OR recurrence_pattern != NONE);

-- Graph edge: user favorited a mosque
DEFINE TABLE IF NOT EXISTS favorited SCHEMAFULL TYPE RELATION IN users OUT mosques;

DEFINE FIELD IF NOT EXISTS created_at ON favorited TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS idx_handles_unique ON handles FIELDS in, out UNIQUE;

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

-- Graph edge: user administers/handles a mosque
DEFINE TABLE IF NOT EXISTS handles SCHEMAFULL TYPE RELATION IN users OUT mosques;

-- When was this permission granted
DEFINE FIELD IF NOT EXISTS granted_at ON handles TYPE datetime DEFAULT time::now();

-- Who granted it (optional audit trail)
DEFINE FIELD IF NOT EXISTS granted_by ON handles TYPE record<users> ASSERT $value != NONE;

-- Prevent duplicate edges
DEFINE INDEX IF NOT EXISTS idx_handles_unique ON handles FIELDS in, out UNIQUE;

-- Graph edge: mosques hosts events
DEFINE TABLE IF NOT EXISTS hosts SCHEMAFULL TYPE RELATION IN mosques OUT events;

DEFINE FIELD IF NOT EXISTS created_at ON hosts TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS created_by ON hosts TYPE record<users> ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS updated_at ON hosts TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS idx_handles_unique ON handles FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS lessons SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON lessons TYPE string;
DEFINE FIELD IF NOT EXISTS module ON lessons TYPE record<modules>;
DEFINE FIELD IF NOT EXISTS content_type ON lessons TYPE string
    ASSERT $value IN ["text", "video", "audio", "pdf", "external_link", "mixed"];
DEFINE FIELD IF NOT EXISTS content ON lessons TYPE string;
DEFINE FIELD IF NOT EXISTS video_url ON lessons TYPE option<string>;
DEFINE FIELD IF NOT EXISTS video_duration_seconds ON lessons TYPE option<int>;
DEFINE FIELD IF NOT EXISTS audio_url ON lessons TYPE option<string>;
DEFINE FIELD IF NOT EXISTS pdf_url ON lessons TYPE option<string>;
DEFINE FIELD IF NOT EXISTS external_url ON lessons TYPE option<string>;
DEFINE FIELD IF NOT EXISTS thumbnail_url ON lessons TYPE option<string>;
DEFINE FIELD IF NOT EXISTS duration_minutes ON lessons TYPE int DEFAULT 5;
DEFINE FIELD IF NOT EXISTS sort_order ON lessons TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS is_preview ON lessons TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS created_at ON lessons TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON lessons TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON lessons TYPE bool DEFAULT false;

DEFINE INDEX IF NOT EXISTS lesson_module_idx ON lessons FIELDS module;

DEFINE TABLE IF NOT EXISTS milestone_courses SCHEMAFULL TYPE RELATION FROM milestones TO courses;

DEFINE FIELD IF NOT EXISTS is_required ON milestone_courses TYPE bool DEFAULT true;

DEFINE INDEX IF NOT EXISTS milestone_courses_unique_idx ON milestone_courses FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS milestones SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS framework ON milestones TYPE record<frameworks>;
DEFINE FIELD IF NOT EXISTS title ON milestones TYPE string;
DEFINE FIELD IF NOT EXISTS description ON milestones TYPE option<string>;
DEFINE FIELD IF NOT EXISTS sort_order ON milestones TYPE int;

DEFINE TABLE IF NOT EXISTS modules SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON modules TYPE string;
DEFINE FIELD IF NOT EXISTS course ON modules TYPE record<courses>;
DEFINE FIELD IF NOT EXISTS description ON modules TYPE option<string>;
DEFINE FIELD IF NOT EXISTS sort_order ON modules TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS created_at ON modules TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON modules TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON modules TYPE bool DEFAULT false;

DEFINE INDEX IF NOT EXISTS module_course_idx ON modules FIELDS course;

DEFINE TABLE IF NOT EXISTS mosques SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON mosques TYPE option<string>;
DEFINE FIELD IF NOT EXISTS location ON mosques TYPE geometry<point>;
DEFINE FIELD IF NOT EXISTS street ON mosques TYPE option<string>;
DEFINE FIELD IF NOT EXISTS city ON mosques TYPE option<string>;
DEFINE FIELD IF NOT EXISTS cover_img ON mosques TYPE option<string>;

DEFINE FIELD IF NOT EXISTS imam ON mosques TYPE option<record<users>>;
DEFINE FIELD IF NOT EXISTS muazzin ON mosques TYPE option<record<users>>;

DEFINE FIELD IF NOT EXISTS adhan_times ON mosques TYPE option<object>;
DEFINE FIELD IF NOT EXISTS adhan_times.fajr ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS adhan_times.dhuhr ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS adhan_times.asr ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS adhan_times.maghrib ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS adhan_times.isha ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS adhan_times.jummah ON mosques TYPE string;

DEFINE FIELD IF NOT EXISTS jamat_times ON mosques TYPE option<object>;
DEFINE FIELD IF NOT EXISTS jamat_times.fajr ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS jamat_times.dhuhr ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS jamat_times.asr ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS jamat_times.maghrib ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS jamat_times.isha ON mosques TYPE string;
DEFINE FIELD IF NOT EXISTS jamat_times.jummah ON mosques TYPE string;

DEFINE FIELD IF NOT EXISTS created_at ON mosques TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON mosques TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS mosque_location_idx ON TABLE mosques COLUMNS location;
DEFINE INDEX IF NOT EXISTS idx_mosque_name ON TABLE mosques COLUMNS name;
DEFINE INDEX IF NOT EXISTS idx_mosque_city ON TABLE mosques COLUMNS city;

DEFINE TABLE IF NOT EXISTS password_reset_tokens SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON password_reset_tokens TYPE record<users> ASSERT $value != NONE;
-- SHA256 hex of the raw reset token. Raw token is only returned once to the requester.
DEFINE FIELD IF NOT EXISTS token_hash ON password_reset_tokens TYPE string ASSERT string::len($value) == 64;
DEFINE FIELD IF NOT EXISTS created_at ON password_reset_tokens TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS expires_at ON password_reset_tokens TYPE datetime;

DEFINE INDEX IF NOT EXISTS idx_reset_token_hash ON TABLE password_reset_tokens COLUMNS token_hash UNIQUE;

DEFINE TABLE IF NOT EXISTS quiz_attempts SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON quiz_attempts TYPE record<users>;
DEFINE FIELD IF NOT EXISTS quiz ON quiz_attempts TYPE record<quizzes>;
DEFINE FIELD IF NOT EXISTS answers ON quiz_attempts TYPE array;
DEFINE FIELD IF NOT EXISTS score ON quiz_attempts TYPE float;
DEFINE FIELD IF NOT EXISTS passed ON quiz_attempts TYPE bool;
DEFINE FIELD IF NOT EXISTS attempted_at ON quiz_attempts TYPE datetime DEFAULT time::now();

DEFINE TABLE IF NOT EXISTS quiz_questions SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS quiz ON quiz_questions TYPE record<quizzes>;
DEFINE FIELD IF NOT EXISTS question_text ON quiz_questions TYPE string;
DEFINE FIELD IF NOT EXISTS question_type ON quiz_questions TYPE string
    ASSERT $value IN ["multiple_choice", "true_false"];
DEFINE FIELD IF NOT EXISTS options ON quiz_questions TYPE option<array<string>>;
DEFINE FIELD IF NOT EXISTS correct_answer ON quiz_questions TYPE string;
DEFINE FIELD IF NOT EXISTS explanation ON quiz_questions TYPE option<string>;
DEFINE FIELD IF NOT EXISTS sort_order ON quiz_questions TYPE int DEFAULT 0;

DEFINE TABLE IF NOT EXISTS quizzes SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS lesson ON quizzes TYPE record<lessons>;
DEFINE FIELD IF NOT EXISTS title ON quizzes TYPE string;
DEFINE FIELD IF NOT EXISTS passing_score ON quizzes TYPE float DEFAULT 0.7;
DEFINE FIELD IF NOT EXISTS max_attempts ON quizzes TYPE int DEFAULT 3;
DEFINE FIELD IF NOT EXISTS created_at ON quizzes TYPE datetime DEFAULT time::now();

DEFINE TABLE IF NOT EXISTS roadmap_courses SCHEMAFULL TYPE RELATION FROM roadmaps TO courses;

DEFINE FIELD IF NOT EXISTS sort_order ON roadmap_courses TYPE int;
DEFINE FIELD IF NOT EXISTS is_required ON roadmap_courses TYPE bool DEFAULT true;
DEFINE FIELD IF NOT EXISTS note ON roadmap_courses TYPE option<string>;

DEFINE INDEX IF NOT EXISTS roadmap_courses_unique_idx ON roadmap_courses FIELDS in, out UNIQUE;

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

DEFINE TABLE OVERWRITE script_migration SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete NONE;

DEFINE FIELD OVERWRITE script_name ON script_migration TYPE string;
DEFINE FIELD OVERWRITE executed_at ON script_migration TYPE datetime VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE checksum ON script_migration TYPE option<string>;

DEFINE TABLE IF NOT EXISTS sessions SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON sessions TYPE record<users> ASSERT $value != NONE;
-- Stores SHA256 hex of the raw session token. Never store raw tokens.
DEFINE FIELD IF NOT EXISTS session_token ON sessions TYPE string ASSERT string::len($value) == 64;
DEFINE FIELD IF NOT EXISTS created_at ON sessions TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS expires_at ON sessions TYPE datetime;

-- Unique Session Tokens (hashed)
DEFINE INDEX IF NOT EXISTS idx_session_token ON TABLE sessions COLUMNS session_token UNIQUE;

DEFINE TABLE IF NOT EXISTS tracks SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON tracks TYPE string;
DEFINE FIELD IF NOT EXISTS slug ON tracks TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD IF NOT EXISTS description ON tracks TYPE string;
DEFINE FIELD IF NOT EXISTS icon ON tracks TYPE option<string>;
DEFINE FIELD IF NOT EXISTS image_url ON tracks TYPE option<string>;
DEFINE FIELD IF NOT EXISTS sort_order ON tracks TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS created_at ON tracks TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON tracks TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted ON tracks TYPE bool DEFAULT false;

DEFINE INDEX IF NOT EXISTS track_slug_idx ON tracks FIELDS slug UNIQUE;

DEFINE TABLE IF NOT EXISTS user_identifier SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON user_identifier TYPE record<users> ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS identifier_type ON user_identifier TYPE string
    ASSERT $value IN ['email', 'mobile', 'google', 'discord', 'microsoft'];
DEFINE FIELD IF NOT EXISTS identifier_value ON user_identifier TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD IF NOT EXISTS created_at ON user_identifier TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON user_identifier TYPE datetime DEFAULT time::now();

-- Allow only one user per identifier_value globally
DEFINE INDEX IF NOT EXISTS idx_identifier_value ON TABLE user_identifier COLUMNS identifier_value UNIQUE;

-- Only one identifier of each type per user
DEFINE INDEX IF NOT EXISTS idx_user_identifier_type ON TABLE user_identifier COLUMNS user, identifier_type UNIQUE;

DEFINE TABLE IF NOT EXISTS user_streaks SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON user_streaks TYPE record<users>;
DEFINE FIELD IF NOT EXISTS current_streak ON user_streaks TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS longest_streak ON user_streaks TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS last_activity_date ON user_streaks TYPE option<string>;
DEFINE FIELD IF NOT EXISTS updated_at ON user_streaks TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS user_streak_user_idx ON user_streaks FIELDS user UNIQUE;

DEFINE TABLE IF NOT EXISTS users SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS password_hash ON users TYPE string
    ASSERT string::len($value) > 0;
DEFINE FIELD IF NOT EXISTS role ON users TYPE string
    ASSERT $value IN ['app_admin', 'mosque_supervisor', 'education_supervisor','educator', 'regular']
    DEFAULT 'regular';
DEFINE FIELD IF NOT EXISTS display_name ON users TYPE string;
DEFINE FIELD IF NOT EXISTS default_mosque ON users TYPE option<record<mosques>>;
DEFINE FIELD IF NOT EXISTS created_at ON users TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON users TYPE datetime DEFAULT time::now();

DEFINE EVENT IF NOT EXISTS cleanup_expired_session ON TABLE sessions WHEN $before != NONE THEN (
    DELETE sessions WHERE expires_at <= time::now()
);
