use chrono::{Datelike, Duration, FixedOffset, TimeZone, Utc};
use merzah::models::events::EventRecurrence;
use merzah::services::recurrence::calculate_next_date;

#[test]
fn test_calculate_next_date_daily() {
    let dt = Utc
        .with_ymd_and_hms(2024, 1, 1, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Daily).unwrap();
    assert_eq!(next, dt + Duration::days(1));
}

#[test]
fn test_calculate_next_date_weekly() {
    let dt = Utc
        .with_ymd_and_hms(2024, 1, 1, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Weekly).unwrap();
    assert_eq!(next, dt + Duration::weeks(1));
}

#[test]
fn test_calculate_next_date_biweekly() {
    let dt = Utc
        .with_ymd_and_hms(2024, 1, 1, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Biweekly).unwrap();
    assert_eq!(next, dt + Duration::weeks(2));
}

#[test]
fn test_calculate_next_date_weekdays() {
    // Monday
    let dt = Utc
        .with_ymd_and_hms(2024, 1, 1, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Weekdays).unwrap();
    assert_eq!(next, dt + Duration::days(1)); // Tuesday

    // Friday
    let dt = Utc
        .with_ymd_and_hms(2024, 1, 5, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Weekdays).unwrap();
    assert_eq!(next, dt + Duration::days(3)); // Monday
}

#[test]
fn test_calculate_next_date_weekends() {
    // Monday
    let dt = Utc
        .with_ymd_and_hms(2024, 1, 1, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Weekends).unwrap();
    assert_eq!(next, dt + Duration::days(5)); // Saturday

    // Saturday
    let dt = Utc
        .with_ymd_and_hms(2024, 1, 6, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Weekends).unwrap();
    assert_eq!(next, dt + Duration::days(1)); // Sunday

    // Sunday
    let dt = Utc
        .with_ymd_and_hms(2024, 1, 7, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Weekends).unwrap();
    assert_eq!(next, dt + Duration::days(6)); // Next Saturday
}

#[test]
fn test_calculate_next_date_monthly() {
    let dt = Utc
        .with_ymd_and_hms(2024, 1, 31, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Monthly).unwrap();
    assert_eq!(next.date_naive().month(), 2);
    assert_eq!(next.date_naive().day(), 29); // 2024 is leap year
}

#[test]
fn test_calculate_next_date_yearly() {
    let dt = Utc
        .with_ymd_and_hms(2024, 2, 29, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Yearly).unwrap();
    assert_eq!(next.date_naive().year(), 2025);
    assert_eq!(next.date_naive().month(), 2);
    assert_eq!(next.date_naive().day(), 28);
}
