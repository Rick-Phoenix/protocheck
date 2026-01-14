use serde::{
  Deserialize, Deserializer, Serialize,
  de::{self, MapAccess, Visitor},
  ser::{SerializeMap, Serializer},
};

use crate::{
  DayOfWeek, String, Vec,
  common::{CalendarPeriod, Month},
  format,
};

impl Serialize for CalendarPeriod {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.as_str_name())
  }
}

impl<'de> Deserialize<'de> for CalendarPeriod {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct CalendarPeriodVisitor;

    impl Visitor<'_> for CalendarPeriodVisitor {
      type Value = CalendarPeriod;

      fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a string representing a CalendarPeriod enum variant")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        CalendarPeriod::from_str_name(v)
          .ok_or_else(|| E::custom(format!("unknown CalendarPeriod variant: {v}")))
      }
    }

    deserializer.deserialize_str(CalendarPeriodVisitor)
  }
}

#[cfg(feature = "datetime")]
impl Serialize for crate::DateTime {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut map = serializer.serialize_map(None)?;

    map.serialize_entry("year", &self.year)?;
    map.serialize_entry("month", &self.month)?;
    map.serialize_entry("day", &self.day)?;
    map.serialize_entry("hours", &self.hours)?;
    map.serialize_entry("minutes", &self.minutes)?;
    map.serialize_entry("seconds", &self.seconds)?;
    map.serialize_entry("nanos", &self.nanos)?;

    match self.time_offset {
      Some(crate::date_time::TimeOffset::UtcOffset(ref d)) => {
        map.serialize_entry("utcOffset", d)?;
      }
      Some(crate::date_time::TimeOffset::TimeZone(ref tz)) => {
        map.serialize_entry("timeZone", tz)?;
      }
      None => {
        // No time_offset, just omit the field as per proto3 JSON spec
      }
    }

    map.end()
  }
}

#[cfg(feature = "datetime")]
impl<'de> Deserialize<'de> for crate::DateTime {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct DateTimeVisitor;

    impl<'de> Visitor<'de> for DateTimeVisitor {
      type Value = crate::DateTime;

      fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a JSON object representing a DateTime")
      }

      fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        let mut year = None;
        let mut month = None;
        let mut day = None;
        let mut hours = None;
        let mut minutes = None;
        let mut seconds = None;
        let mut nanos = None;
        let mut utc_offset = None;
        let mut time_zone = None;

        while let Some(key) = map.next_key::<String>()? {
          match key.as_str() {
            "year" => year = Some(map.next_value()?),
            "month" => month = Some(map.next_value()?),
            "day" => day = Some(map.next_value()?),
            "hours" => hours = Some(map.next_value()?),
            "minutes" => minutes = Some(map.next_value()?),
            "seconds" => seconds = Some(map.next_value()?),
            "nanos" => nanos = Some(map.next_value()?),
            "utcOffset" => {
              if time_zone.is_some() {
                return Err(de::Error::custom(
                  "found 'utcOffset' and 'timeZone', expected only one",
                ));
              }
              utc_offset = Some(map.next_value()?);
            }
            "timeZone" => {
              if utc_offset.is_some() {
                return Err(de::Error::custom(
                  "found 'utcOffset' and 'timeZone', expected only one",
                ));
              }
              time_zone = Some(map.next_value()?);
            }
            _ => {
              let _ = map.next_value::<serde_json::Value>()?;
            }
          }
        }

        let time_offset = match (utc_offset, time_zone) {
          (Some(d), None) => Some(crate::date_time::TimeOffset::UtcOffset(d)),
          (None, Some(tz)) => Some(crate::date_time::TimeOffset::TimeZone(tz)),
          (None, None) => None,
          _ => {
            return Err(de::Error::custom(
              "found 'utcOffset' and 'timeZone', expected only one",
            ));
          }
        };

        let month_val = month.ok_or_else(|| de::Error::missing_field("month"))?;
        let day_val = day.ok_or_else(|| de::Error::missing_field("day"))?;
        let hours_val = hours.ok_or_else(|| de::Error::missing_field("hours"))?;
        let minutes_val = minutes.ok_or_else(|| de::Error::missing_field("minutes"))?;
        let seconds_val = seconds.ok_or_else(|| de::Error::missing_field("seconds"))?;
        let nanos_val = nanos.ok_or_else(|| de::Error::missing_field("nanos"))?;

        Ok(crate::DateTime {
          year: year.unwrap_or(0), // Year is optional, default 0
          month: month_val,
          day: day_val,
          hours: hours_val,
          minutes: minutes_val,
          seconds: seconds_val,
          nanos: nanos_val,
          time_offset,
        })
      }
    }

    deserializer.deserialize_map(DateTimeVisitor)
  }
}

#[cfg(feature = "date")]
impl Serialize for crate::Date {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let date_string = format!("{:04}-{:02}-{:02}", self.year, self.month, self.day);
    serializer.serialize_str(&date_string)
  }
}

#[cfg(feature = "date")]
impl<'de> Deserialize<'de> for crate::Date {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct DateVisitor;

    impl Visitor<'_> for DateVisitor {
      type Value = crate::Date;

      fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a date string in YYYY-MM-DD format")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        let parts: Vec<&str> = v.split('-').collect();

        if parts.len() != 3 {
          return Err(E::custom(format!(
            "invalid date format: {v}, expected YYYY-MM-DD"
          )));
        }

        let year = parts[0].parse::<i32>().map_err(E::custom)?;
        let month = parts[1].parse::<i32>().map_err(E::custom)?;
        let day = parts[2].parse::<i32>().map_err(E::custom)?;

        if !(1..=12).contains(&month) {
          return Err(E::custom(format!("invalid month: {month}")));
        }
        if !(1..=31).contains(&day) {
          return Err(E::custom(format!("invalid day: {day}")));
        }

        Ok(crate::Date { year, month, day })
      }
    }

    deserializer.deserialize_str(DateVisitor)
  }
}

impl Serialize for DayOfWeek {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.as_str_name())
  }
}

impl<'de> Deserialize<'de> for DayOfWeek {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct DayOfWeekVisitor;

    impl Visitor<'_> for DayOfWeekVisitor {
      type Value = DayOfWeek;

      fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a string representing a DayOfWeek enum variant (e.g., \"MONDAY\")")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        DayOfWeek::from_str_name(v)
          .ok_or_else(|| E::custom(format!("unknown DayOfWeek variant: {v}")))
      }
    }

    deserializer.deserialize_str(DayOfWeekVisitor)
  }
}

impl Serialize for Month {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.as_str_name())
  }
}

impl<'de> Deserialize<'de> for Month {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct MonthVisitor;

    impl Visitor<'_> for MonthVisitor {
      type Value = Month;

      fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a string representing a Month enum variant (e.g., \"JANUARY\")")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Month::from_str_name(v).ok_or_else(|| E::custom(format!("unknown Month variant: {v}")))
      }
    }

    deserializer.deserialize_str(MonthVisitor)
  }
}
