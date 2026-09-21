use chrono::NaiveTime;
use chrono::TimeDelta;

use crate::models::api_responses::MosqueResponse;
use crate::models::mosque::PrayerTimes;
use crate::models::prayer::NextPrayerCardData;
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

/// `"19:42"` -> `"7:42 PM"`.
pub fn format_iqamah(time: NaiveTime) -> String {
    time.format("%-I:%M %p").to_string()
}

pub fn pad2(value: u64) -> String {
    format!("{value:02}")
}

/// Builds everything `NextPrayerReminderCard` needs from one mosque.
///
/// Missing `jamat_times` yields a `has_times: false` placeholder instead of
/// a countdown, so the page never has to branch on prayer math itself.
pub fn next_prayer_card_data(
    mosque: &MosqueResponse,
    now: NaiveTime,
    is_friday: bool,
) -> NextPrayerCardData {
    let mosque_name = mosque
        .name
        .clone()
        .unwrap_or_else(|| "Nearby mosque".to_string());
    let location = mosque
        .city
        .clone()
        .unwrap_or_else(|| "Nearby".to_string());

    match mosque.jamat_times.as_ref() {
        None => NextPrayerCardData::without_times(mosque_name, location),
        Some(jamat) => {
            let info = next_jamat(jamat, now, is_friday);
            let total_seconds = info.hours * 3600 + info.minutes * 60 + info.seconds;

            NextPrayerCardData::new(
                mosque_name,
                location,
                info.name.to_string(),
                format_iqamah(info.time),
                pad2(info.hours),
                pad2(info.minutes),
                pad2(info.seconds),
                total_seconds,
                true,
            )
        }
    }
}
