use thiserror::Error;

use crate::{
  common::{date_time, DateTime, TimeZone},
  Duration,
};

#[derive(Debug, PartialEq, Eq, Error)]
pub enum DateTimeError {
  #[error("DateTime has an invalid date component (e.g., February 30th)")]
  InvalidDate,
  #[error(
    "DateTime has an invalid time component (e.g., hours, minutes, seconds, nanos out of range)"
  )]
  InvalidTime,
  #[error("DateTime arithmetic resulted in a time outside its representable range")]
  OutOfRange,
  #[error("DateTime conversion error: {0}")]
  ConversionError(String),
}

impl PartialOrd for TimeZone {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for TimeZone {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self
      .id
      .cmp(&other.id)
      .then_with(|| self.version.cmp(&other.version))
  }
}

impl PartialOrd for date_time::TimeOffset {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for date_time::TimeOffset {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    match (self, other) {
      (Self::UtcOffset(a), Self::UtcOffset(b)) => a.cmp(b),
      (Self::TimeZone(a), Self::TimeZone(b)) => a.cmp(b),
      // Define an arbitrary total ordering between different variants:
      // UtcOffset comes before TimeZone
      (Self::UtcOffset(_), Self::TimeZone(_)) => std::cmp::Ordering::Less,
      (Self::TimeZone(_), Self::UtcOffset(_)) => std::cmp::Ordering::Greater,
    }
  }
}

impl PartialOrd for DateTime {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for DateTime {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self
      .year
      .cmp(&other.year)
      .then_with(|| self.month.cmp(&other.month))
      .then_with(|| self.day.cmp(&other.day))
      .then_with(|| self.hours.cmp(&other.hours))
      .then_with(|| self.minutes.cmp(&other.minutes))
      .then_with(|| self.seconds.cmp(&other.seconds))
      .then_with(|| self.nanos.cmp(&other.nanos))
      .then_with(|| self.time_offset.cmp(&other.time_offset))
  }
}

#[allow(clippy::too_many_arguments)]
fn datetime_is_valid(
  year: i32,
  month: i32,
  day: i32,
  hours: i32,
  minutes: i32,
  seconds: i32,
  nanos: i32,
) -> Result<(), DateTimeError> {
  if year != 0 && !(1..=9999).contains(&year) {
    return Err(DateTimeError::InvalidDate);
  }
  if !(1..=12).contains(&month) {
    return Err(DateTimeError::InvalidDate);
  }
  if !(1..=31).contains(&day) {
    return Err(DateTimeError::InvalidDate);
  }

  if !(0..=23).contains(&hours) {
    return Err(DateTimeError::InvalidTime);
  }
  if !(0..=59).contains(&minutes) {
    return Err(DateTimeError::InvalidTime);
  }
  if !(0..=59).contains(&seconds) {
    return Err(DateTimeError::InvalidTime);
  }
  if !(0..=999_999_999).contains(&nanos) {
    return Err(DateTimeError::InvalidTime);
  }

  Ok(())
}

impl DateTime {
  /// Checks if this `DateTime` instance represents a valid date and time, and returns the related error if it does not.
  pub fn validate(&self) -> Result<(), DateTimeError> {
    datetime_is_valid(
      self.year,
      self.month,
      self.day,
      self.hours,
      self.minutes,
      self.seconds,
      self.nanos,
    )
  }

  /// Checks if this `DateTime` instance represents a valid date and time.
  pub fn is_valid(&self) -> bool {
    self.validate().is_ok()
  }

  /// Returns `true` if the `DateTime` has a specific year (i.e., `year` is not 0).
  pub fn has_year(&self) -> bool {
    self.year != 0
  }

  /// Sets the `time_offset` to a UTC offset `Duration`, clearing any existing time zone.
  pub fn with_utc_offset(mut self, offset: Duration) -> Self {
    self.time_offset = Some(date_time::TimeOffset::UtcOffset(offset));
    self
  }

  /// Sets the `time_offset` to a `TimeZone`, clearing any existing UTC offset.
  pub fn with_time_zone(mut self, time_zone: TimeZone) -> Self {
    self.time_offset = Some(date_time::TimeOffset::TimeZone(time_zone));
    self
  }
}

#[cfg(feature = "chrono")]
impl TryFrom<DateTime> for chrono::NaiveDateTime {
  type Error = DateTimeError;

  fn try_from(dt: DateTime) -> Result<Self, Self::Error> {
    // NaiveDateTime does not support year 0, nor does it carry time offset.
    if dt.year == 0 {
      return Err(DateTimeError::ConversionError(
        "Cannot convert DateTime with year 0 to NaiveDateTime".to_string(),
      ));
    }
    if dt.time_offset.is_some() {
      return Err(DateTimeError::ConversionError(
               "Cannot convert DateTime with explicit time offset to NaiveDateTime without losing information".to_string()
           ));
    }

    dt.validate()?;

    let date = chrono::NaiveDate::from_ymd_opt(dt.year, dt.month as u32, dt.day as u32)
      .ok_or(DateTimeError::InvalidDate)?;
    let time = chrono::NaiveTime::from_hms_nano_opt(
      dt.hours as u32,
      dt.minutes as u32,
      dt.seconds as u32,
      dt.nanos as u32,
    )
    .ok_or(DateTimeError::InvalidTime)?;

    Ok(chrono::NaiveDateTime::new(date, time))
  }
}

#[cfg(feature = "chrono")]
impl TryFrom<DateTime> for chrono::DateTime<chrono::Utc> {
  type Error = DateTimeError;
  fn try_from(value: DateTime) -> Result<Self, Self::Error> {
    match &value.time_offset {
      Some(date_time::TimeOffset::UtcOffset(proto_duration)) => {
        if !(proto_duration.seconds == 0 && proto_duration.nanos == 0) {
          return Err(DateTimeError::ConversionError(
            "Cannot convert DateTime to TimeZone<Utc> when the UtcOffset is not 0.".to_string(),
          ));
        }
      }
      Some(date_time::TimeOffset::TimeZone(_)) => {
        return Err(DateTimeError::ConversionError(
          "Cannot convert DateTime to TimeZone<Utc> when a UtcOffset is not set.".to_string(),
        ))
      }
      None => {
        return Err(DateTimeError::ConversionError(
          "Cannot convert DateTime to TimeZone<Utc> when a UtcOffset is not set.".to_string(),
        ))
      }
    };

    let naive_dt: chrono::NaiveDateTime = value.try_into()?;

    Ok(naive_dt.and_utc())
  }
}

#[cfg(feature = "chrono")]
impl TryFrom<DateTime> for chrono::DateTime<chrono::FixedOffset> {
  type Error = DateTimeError;
  fn try_from(value: DateTime) -> Result<Self, Self::Error> {
    let offset = match &value.time_offset {
      Some(date_time::TimeOffset::UtcOffset(proto_duration)) => {
        use crate::constants::NANOS_PER_SECOND;

        let nanos_to_seconds = proto_duration.nanos / NANOS_PER_SECOND;
        let total_seconds: i32 = (proto_duration.seconds * nanos_to_seconds as i64)
          .try_into()
          .or(Err(DateTimeError::ConversionError(
            "UtcOffset is outside of the allowed range.".to_string(),
          )))?;

        chrono::FixedOffset::east_opt(total_seconds).ok_or_else(|| {
          DateTimeError::ConversionError(
            "Failed to convert proto::Duration to chrono::FixedOffset due to invalid offset values"
              .to_string(),
          )
        })
      }
      Some(date_time::TimeOffset::TimeZone(_)) => Err(DateTimeError::ConversionError(
        "Cannot convert DateTime with named TimeZone to FixedOffset".to_string(),
      )),
      None => Err(DateTimeError::ConversionError(
        "Cannot convert local DateTime to FixedOffset without explicit offset".to_string(),
      )),
    }?;

    let naive_dt: chrono::NaiveDateTime = value.try_into()?;

    naive_dt
      .and_local_timezone(offset)
      .single() // Take the unique result if not ambiguous
      .ok_or(DateTimeError::ConversionError(
        "Ambiguous or invalid local time to FixedOffset conversion".to_string(),
      ))
  }
}

#[cfg(feature = "chrono")]
impl From<chrono::DateTime<chrono::Utc>> for DateTime {
  fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
    use chrono::{Datelike, Timelike};
    DateTime {
      year: value.year(),
      month: value.month() as i32,
      day: value.day() as i32,
      hours: value.hour() as i32,
      minutes: value.minute() as i32,
      seconds: value.second() as i32,
      nanos: value.nanosecond() as i32,
      time_offset: Some(date_time::TimeOffset::UtcOffset(Duration::new(0, 0))),
    }
  }
}

#[cfg(feature = "chrono")]
impl From<chrono::NaiveDateTime> for DateTime {
  fn from(ndt: chrono::NaiveDateTime) -> Self {
    use chrono::{Datelike, Timelike};

    // NaiveDateTime has no offset, so DateTime will be local time
    DateTime {
      year: ndt.year(),
      month: ndt.month() as i32,
      day: ndt.day() as i32,
      hours: ndt.hour() as i32,
      minutes: ndt.minute() as i32,
      seconds: ndt.second() as i32,
      nanos: ndt.nanosecond() as i32,
      time_offset: None,
    }
  }
}
