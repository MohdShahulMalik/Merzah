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

#[test]
fn test_calculate_next_date_quarterly() {
    let dt = Utc
        .with_ymd_and_hms(2024, 1, 15, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Quaterly).unwrap();
    assert_eq!(next.date_naive().month(), 4);
    assert_eq!(next.date_naive().day(), 15);

    let next2 = calculate_next_date(next, EventRecurrence::Quaterly).unwrap();
    assert_eq!(next2.date_naive().month(), 7);
    assert_eq!(next2.date_naive().day(), 15);

    let next3 = calculate_next_date(next2, EventRecurrence::Quaterly).unwrap();
    assert_eq!(next3.date_naive().month(), 10);
    assert_eq!(next3.date_naive().day(), 15);
}

#[test]
fn test_calculate_next_date_quarterly_year_boundary() {
    let dt = Utc
        .with_ymd_and_hms(2024, 11, 15, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Quaterly).unwrap();
    assert_eq!(next.date_naive().year(), 2025);
    assert_eq!(next.date_naive().month(), 2);
    assert_eq!(next.date_naive().day(), 15);
}

#[test]
fn test_calculate_next_date_monthly_boundary_jan_to_feb() {
    let dt = Utc
        .with_ymd_and_hms(2024, 1, 31, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Monthly).unwrap();
    assert_eq!(next.date_naive().month(), 2);
    assert_eq!(next.date_naive().day(), 29);
}

#[test]
fn test_calculate_next_date_monthly_boundary_dec_to_jan() {
    let dt = Utc
        .with_ymd_and_hms(2024, 12, 31, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Monthly).unwrap();
    assert_eq!(next.date_naive().year(), 2025);
    assert_eq!(next.date_naive().month(), 1);
    assert_eq!(next.date_naive().day(), 31);
}

#[test]
fn test_calculate_next_date_monthly_non_leap_year() {
    let dt = Utc
        .with_ymd_and_hms(2023, 1, 31, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Monthly).unwrap();
    assert_eq!(next.date_naive().month(), 2);
    assert_eq!(next.date_naive().day(), 28);
}

#[test]
fn test_calculate_next_date_yearly_leap_to_non_leap() {
    let dt = Utc
        .with_ymd_and_hms(2024, 2, 29, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Yearly).unwrap();
    assert_eq!(next.date_naive().year(), 2025);
    assert_eq!(next.date_naive().month(), 2);
    assert_eq!(next.date_naive().day(), 28);

    let next2 = calculate_next_date(next, EventRecurrence::Yearly).unwrap();
    assert_eq!(next2.date_naive().year(), 2026);
    assert_eq!(next2.date_naive().month(), 2);
    assert_eq!(next2.date_naive().day(), 28);
}

#[test]
fn test_calculate_next_date_timezone_positive_offset() {
    let offset = FixedOffset::east_opt(5 * 3600).unwrap();
    let dt = offset.with_ymd_and_hms(2024, 1, 1, 10, 0, 0).unwrap();
    let next = calculate_next_date(dt, EventRecurrence::Daily).unwrap();
    assert_eq!(next.date_naive(), dt.date_naive() + Duration::days(1));
    assert_eq!(next.time(), dt.time());
}

#[test]
fn test_calculate_next_date_timezone_negative_offset() {
    let offset = FixedOffset::west_opt(8 * 3600).unwrap();
    let dt = offset.with_ymd_and_hms(2024, 1, 1, 10, 0, 0).unwrap();
    let next = calculate_next_date(dt, EventRecurrence::Daily).unwrap();
    assert_eq!(next.date_naive(), dt.date_naive() + Duration::days(1));
    assert_eq!(next.time(), dt.time());
}

#[test]
fn test_calculate_next_date_preserves_time() {
    let dt = Utc
        .with_ymd_and_hms(2024, 1, 1, 14, 30, 45)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());

    let next = calculate_next_date(dt, EventRecurrence::Daily).unwrap();
    assert_eq!(next.time(), dt.time());

    let next = calculate_next_date(dt, EventRecurrence::Weekly).unwrap();
    assert_eq!(next.time(), dt.time());

    let next = calculate_next_date(dt, EventRecurrence::Monthly).unwrap();
    assert_eq!(next.time(), dt.time());

    let next = calculate_next_date(dt, EventRecurrence::Yearly).unwrap();
    assert_eq!(next.time(), dt.time());
}

#[test]
fn test_calculate_next_date_monthly_30_day_month_to_31_day() {
    let dt = Utc
        .with_ymd_and_hms(2024, 4, 30, 10, 0, 0)
        .unwrap()
        .with_timezone(&FixedOffset::east_opt(0).unwrap());
    let next = calculate_next_date(dt, EventRecurrence::Monthly).unwrap();
    assert_eq!(next.date_naive().month(), 5);
    assert_eq!(next.date_naive().day(), 30);
}

#[test]
fn test_weekdays_all_days() {
    let test_cases = [
        (2024, 1, 1, 1),
        (2024, 1, 2, 1),
        (2024, 1, 3, 1),
        (2024, 1, 4, 1),
        (2024, 1, 5, 3),
    ];

    for (year, month, day, expected_days) in test_cases {
        let dt = Utc
            .with_ymd_and_hms(year, month, day, 10, 0, 0)
            .unwrap()
            .with_timezone(&FixedOffset::east_opt(0).unwrap());
        let next = calculate_next_date(dt, EventRecurrence::Weekdays).unwrap();
        assert_eq!(
            next,
            dt + Duration::days(expected_days),
            "Failed for {}-{}-{}",
            year,
            month,
            day
        );
    }
}

#[test]
fn test_weekends_all_days() {
    let test_cases = [
        (2024, 1, 1, 5),
        (2024, 1, 2, 4),
        (2024, 1, 3, 3),
        (2024, 1, 4, 2),
        (2024, 1, 5, 1),
        (2024, 1, 6, 1),
        (2024, 1, 7, 6),
    ];

    for (year, month, day, expected_days) in test_cases {
        let dt = Utc
            .with_ymd_and_hms(year, month, day, 10, 0, 0)
            .unwrap()
            .with_timezone(&FixedOffset::east_opt(0).unwrap());
        let next = calculate_next_date(dt, EventRecurrence::Weekends).unwrap();
        assert_eq!(
            next,
            dt + Duration::days(expected_days),
            "Failed for {}-{}-{}",
            year,
            month,
            day
        );
    }
}
