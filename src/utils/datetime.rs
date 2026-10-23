#[cfg(feature = "ssr")]
use chrono::{DateTime, FixedOffset, Utc};
#[cfg(feature = "ssr")]
use surrealdb::Datetime;

/// Converts a datetime coming from the database into a fixed offset datetime,
/// falling back to the current time when the stored value is not parsable.
#[cfg(feature = "ssr")]
pub fn datetime_to_fixed(datetime: Datetime) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(&datetime.to_string())
        .unwrap_or_else(|_| Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()))
}
