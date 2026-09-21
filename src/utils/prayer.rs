use chrono::NaiveTime;
use chrono::TimeDelta;

use crate::models::mosque::PrayerTimes;
use crate::models::prayer::NextPrayerInfo;

/// First jamat strictly after `now`; wraps to tomorrow's Fajr.
///
/// On Fridays `jummah` replaces `dhuhr`.
pub fn next_jamat(jamat: &PrayerTimes, now: NaiveTime, is_friday: bool) -> NextPrayerInfo {
    let mut ordered: Vec<(&'static str, NaiveTime)> = if is_friday {
        vec![
            ("Fajr", jamat.fajr),
            ("Jumu'ah", jamat.jummah),
            ("Asr", jamat.asr),
            ("Maghrib", jamat.maghrib),
            ("Isha", jamat.isha),
        ]
    } else {
        vec![
            ("Fajr", jamat.fajr),
            ("Dhuhr", jamat.dhuhr),
            ("Asr", jamat.asr),
            ("Maghrib", jamat.maghrib),
            ("Isha", jamat.isha),
        ]
    };
    ordered.sort_by_key(|(_, time)| *time);

    let first = ordered[0];
    let (name, time, is_tomorrow) = ordered
        .into_iter()
        .find(|(_, time)| *time > now)
        .map(|(name, time)| (name, time, false))
        .unwrap_or((first.0, first.1, true));

    let (hours, minutes, seconds) = remaining_hms(time, now);

    NextPrayerInfo::new(name, time, is_tomorrow, hours, minutes, seconds)
}

/// Hours/minutes/seconds from `now` until `target`, wrapping past midnight.
///
/// Returns `(hours, minutes, seconds)`: whole hours in the wait, leftover
/// whole minutes after those hours, leftover seconds after those minutes.
pub fn remaining_hms(target: NaiveTime, now: NaiveTime) -> (u64, u64, u64) {
    let delta = target.signed_duration_since(now);
    let delta = if delta < TimeDelta::zero() {
        delta + TimeDelta::hours(24)
    } else {
        delta
    };
    let total = delta.num_seconds().max(0) as u64;

    (total / 3600, (total % 3600) / 60, total % 60)
}
