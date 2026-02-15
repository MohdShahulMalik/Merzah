DEFINE TABLE IF NOT EXISTS events SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS title ON events TYPE string;
DEFINE FIELD IF NOT EXISTS description ON events TYPE string;
DEFINE FIELD IF NOT EXISTS category ON events TYPE string;
DEFINE FIELD IF NOT EXISTS date ON events TYPE datetime;
DEFINE FIELD IF NOT EXISTS speaker ON events TYPE option<string>;

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
