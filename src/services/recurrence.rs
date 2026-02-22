use chrono::{
    DateTime, Datelike, Duration, FixedOffset, LocalResult, NaiveDate, NaiveDateTime, TimeZone,
};
use std::cmp::min;

#[cfg(feature = "ssr")]
use crate::models::events::Event;
use crate::models::events::EventRecurrence;
#[cfg(feature = "ssr")]
use surrealdb::{engine::remote::ws::Client, Surreal};

pub fn calculate_next_date(
    curr_date: DateTime<FixedOffset>,
    pattern: EventRecurrence,
) -> Option<DateTime<FixedOffset>> {
    match pattern {
        EventRecurrence::Daily => Some(curr_date + Duration::days(1)),

        EventRecurrence::Weekly => Some(curr_date + Duration::weeks(1)),

        EventRecurrence::Biweekly => Some(curr_date + Duration::weeks(2)),

        EventRecurrence::Weekdays => {
            let weekday = curr_date.weekday().number_from_monday();
            let days_to_add = if weekday >= 5 { 8 - weekday } else { 1 };
            Some(curr_date + Duration::days(days_to_add as i64))
        }

        EventRecurrence::Weekends => {
            let weekday = curr_date.weekday().number_from_monday();
            let days_to_add = if weekday <= 5 {
                6 - weekday
            } else if weekday == 6 {
                1
            } else {
                6
            };
            Some(curr_date + Duration::days(days_to_add as i64))
        }

        EventRecurrence::Monthly => {
            let date = curr_date.date_naive();
            let next_month = if date.month() == 12 {
                1
            } else {
                date.month() + 1
            };
            let year = if next_month == 1 {
                date.year() + 1
            } else {
                date.year()
            };
            let day = min(date.day(), days_in_month(year, next_month));

            let next_date: NaiveDate = NaiveDate::from_ymd_opt(year, next_month, day)
                .or_else(|| NaiveDate::from_ymd_opt(year, next_month, 1))?;

            let naive_datetime: NaiveDateTime = next_date.and_time(curr_date.time());

            match curr_date.timezone().from_local_datetime(&naive_datetime) {
                LocalResult::Single(dt) => Some(dt),
                _ => None,
            }
        }

        EventRecurrence::Quaterly => {
            let date = curr_date.date_naive();
            let months_to_add = 3;
            let total_months = (date.year() * 12) + date.month() as i32;
            let next_total_months = total_months + months_to_add;
            let next_year = next_total_months / 12;
            let next_month = (next_total_months % 12) as u32;
            let next_month = if next_month == 0 { 12 } else { next_month };
            let day = min(date.day(), days_in_month(next_year, next_month));

            let next_date = NaiveDate::from_ymd_opt(next_year, next_month, day)
                .or_else(|| NaiveDate::from_ymd_opt(next_year, next_month, 1))?;
            let naive_datetime = next_date.and_time(curr_date.time());

            match curr_date.timezone().from_local_datetime(&naive_datetime) {
                LocalResult::Single(dt) => Some(dt),
                _ => None,
            }
        }

        EventRecurrence::Yearly => {
            let date = curr_date.date_naive();
            let next_year = date.year() + 1;
            let day = min(date.day(), days_in_month(next_year, date.month()));

            let next_date = NaiveDate::from_ymd_opt(next_year, date.month(), day)
                .or_else(|| NaiveDate::from_ymd_opt(next_year, date.month(), 1))?;
            let naive_datetime = next_date.and_time(curr_date.time());

            match curr_date.timezone().from_local_datetime(&naive_datetime) {
                LocalResult::Single(dt) => Some(dt),
                _ => None,
            }
        }
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,

        4 | 6 | 9 | 11 => 30,

        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        },
        
        _ => 30,
    }
}

#[cfg(feature = "ssr")]
pub async fn rotate_event(event: Event, db: &Surreal<Client>) -> Result<bool, surrealdb::Error> {
    use tracing::{error, info};

    let Some(pattern) = event.recurrence_pattern.clone() else {
        return Ok(false);
    };

    let Some(next_date) = calculate_next_date(event.date, pattern) else {
        error!("Failed to calculate next date for event {}", event.id);
        return Ok(false);
    };

    if let Some(end_date) = event.recurrence_end_date {
        if next_date > end_date {
            db.query("DELETE $event")
                .bind(("event", event.id.clone()))
                .await?;
            info!("Deleted event {} - recurrence series ended", event.id);
            return Ok(false);
        }
    }

    db.query("UPDATE $event SET date = $next_date")
        .bind(("event", event.id.clone()))
        .bind(("next_date", next_date.to_rfc3339()))
        .await?;

    info!("Rotated event {} to {}", event.id, next_date);
    Ok(true)
}

#[cfg(feature = "ssr")]
pub async fn check_and_rotate_events(db: &Surreal<Client>) -> Result<usize, surrealdb::Error> {
    use tracing::{error, info};

    let search_query = r#"
        SELECT * FROM events
        WHERE time::from::iso8601(date) < time::now()
        AND recurrence_pattern != NONE
    "#;

    let events: Vec<Event> = db
        .query(search_query)
        .await?
        .take(0)?;

    let mut rotated_count = 0;

    for event in events {
        match rotate_event(event, db).await {
            Ok(true) => rotated_count += 1,
            Ok(false) => {}
            Err(e) => error!("Failed to rotate event: {}", e),
        }
    }

    info!("Rotated {} events", rotated_count);
    Ok(rotated_count)
}
