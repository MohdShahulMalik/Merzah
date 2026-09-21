use chrono::NaiveTime;
use merzah::models::mosque::PrayerTimes;
use merzah::utils::prayer::format_iqamah;
use merzah::utils::prayer::next_jamat;
use merzah::utils::prayer::remaining_hms;
use rstest::rstest;

fn sample_jamat() -> PrayerTimes {
    PrayerTimes {
        fajr: NaiveTime::from_hms_opt(5, 30, 0).unwrap(),
        dhuhr: NaiveTime::from_hms_opt(13, 30, 0).unwrap(),
        asr: NaiveTime::from_hms_opt(17, 15, 0).unwrap(),
        maghrib: NaiveTime::from_hms_opt(19, 42, 0).unwrap(),
        isha: NaiveTime::from_hms_opt(21, 15, 0).unwrap(),
        jummah: NaiveTime::from_hms_opt(13, 15, 0).unwrap(),
    }
}

fn at(hour: u32, min: u32) -> NaiveTime {
    NaiveTime::from_hms_opt(hour, min, 0).unwrap()
}

#[rstest]
#[case::mid_morning(at(6, 0), false, "Dhuhr", false)]
#[case::mid_afternoon(at(14, 0), false, "Asr", false)]
#[case::after_isha_wraps_to_tomorrow(at(22, 0), false, "Fajr", true)]
#[case::exact_time_moves_to_next(at(13, 30), false, "Asr", false)]
#[case::friday_replaces_dhuhr(at(12, 0), true, "Jumu'ah", false)]
#[case::friday_afternoon(at(14, 0), true, "Asr", false)]
fn picks_next_jamat(
    #[case] now: NaiveTime,
    #[case] is_friday: bool,
    #[case] expected_name: &'static str,
    #[case] expected_tomorrow: bool,
) {
    let info = next_jamat(&sample_jamat(), now, is_friday);

    assert_eq!(info.name, expected_name);
    assert_eq!(info.is_tomorrow, expected_tomorrow);
}

#[rstest]
#[case::same_day(at(19, 42), at(17, 0), (2, 42, 0))]
#[case::wraps_past_midnight(at(5, 30), at(22, 0), (7, 30, 0))]
#[case::zero_when_equal(at(13, 30), at(13, 30), (0, 0, 0))]
fn counts_down_to_target(
    #[case] target: NaiveTime,
    #[case] now: NaiveTime,
    #[case] expected: (u64, u64, u64),
) {
    assert_eq!(remaining_hms(target, now), expected);
}

#[rstest]
#[case::evening(at(19, 42), "7:42 PM")]
#[case::morning(at(5, 30), "5:30 AM")]
fn formats_iqamah(#[case] time: NaiveTime, #[case] expected: &str) {
    assert_eq!(format_iqamah(time), expected);
}
