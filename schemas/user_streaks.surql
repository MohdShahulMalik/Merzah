DEFINE TABLE IF NOT EXISTS user_streaks SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user ON user_streaks TYPE record<users>;
DEFINE FIELD IF NOT EXISTS current_streak ON user_streaks TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS longest_streak ON user_streaks TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS last_activity_date ON user_streaks TYPE option<string>;
DEFINE FIELD IF NOT EXISTS updated_at ON user_streaks TYPE datetime DEFAULT time::now();

DEFINE INDEX IF NOT EXISTS user_streak_user_idx ON user_streaks FIELDS user UNIQUE;
