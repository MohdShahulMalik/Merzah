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
