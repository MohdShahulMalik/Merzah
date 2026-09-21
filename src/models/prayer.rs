use chrono::NaiveTime;

/// Next iqamah with its countdown components.
#[derive(Debug, Clone, PartialEq)]
pub struct NextPrayerInfo {
    pub name: &'static str,
    pub time: NaiveTime,
    pub is_tomorrow: bool,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}

impl NextPrayerInfo {
    pub fn new(
        name: &'static str,
        time: NaiveTime,
        is_tomorrow: bool,
        hours: u64,
        minutes: u64,
        seconds: u64,
    ) -> Self {
        Self {
            name,
            time,
            is_tomorrow,
            hours,
            minutes,
            seconds,
        }
    }
}

/// Render-ready strings for `NextPrayerReminderCard`.
#[derive(Debug, Clone, PartialEq)]
pub struct NextPrayerCardData {
    pub mosque_name: String,
    pub location: String,
    pub prayer_name: String,
    pub iqamah_time: String,
    pub hours: String,
    pub minutes: String,
    pub seconds: String,
    pub total_seconds: u64,
    pub has_times: bool,
}

impl NextPrayerCardData {
    pub fn new(
        mosque_name: String,
        location: String,
        prayer_name: String,
        iqamah_time: String,
        hours: String,
        minutes: String,
        seconds: String,
        total_seconds: u64,
        has_times: bool,
    ) -> Self {
        Self {
            mosque_name,
            location,
            prayer_name,
            iqamah_time,
            hours,
            minutes,
            seconds,
            total_seconds,
            has_times,
        }
    }

    pub fn without_times(mosque_name: String, location: String) -> Self {
        Self {
            mosque_name,
            location,
            prayer_name: "Times not set".to_string(),
            iqamah_time: "--".to_string(),
            hours: "00".to_string(),
            minutes: "00".to_string(),
            seconds: "00".to_string(),
            total_seconds: 0,
            has_times: false,
        }
    }
}
