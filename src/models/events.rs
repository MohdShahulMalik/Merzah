use serde::{ Deserialize, Serialize, Serializer, Deserializer };
use chrono::{ DateTime, NaiveDateTime, TimeZone, Utc };

#[derive(Debug, Deserialize, Serialize)]
pub struct Events {
    title: String,
    description: String,
    category: String,
    #[serde(
        serialize_with = "to_surreal_datetime",
        deserialize_with = "from_surreal_datetime"
    )]
    date: NaiveDateTime,
}

fn to_surreal_datetime<S>(date: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let datetime = Utc.from_utc_datetime(date);
    datetime.serialize(s)
}

fn from_surreal_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let dt = DateTime::<Utc>::deserialize(deserializer)?;
    Ok(dt.naive_utc())
}
