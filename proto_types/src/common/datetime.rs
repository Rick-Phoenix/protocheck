use std::fmt::{Display, Formatter};

use thiserror::Error;

use crate::{
  common::{date_time::TimeOffset, DateTime, TimeZone},
  Duration,
};

impl Display for TimeZone {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.id)
  }
}

impl Display for DateTime {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.year != 0 {
      write!(f, "{:04}-", self.year)?;
    }
    write!(f, "{:02}-{:02}", self.month, self.day)?;

    write!(
      f,
      "T{:02}:{:02}:{:02}",
      self.hours, self.minutes, self.seconds
    )?;

    match &self.time_offset {
      Some(TimeOffset::UtcOffset(duration)) => {
        let total_offset_seconds = duration.normalized().seconds;
        let is_negative = total_offset_seconds < 0;
        let abs_total_offset_seconds = total_offset_seconds.abs();

        let hours = abs_total_offset_seconds / 3600;
        let minutes = (abs_total_offset_seconds % 3600) / 60;

        if is_negative {
          write!(f, "-{:02}:{:02}", hours, minutes)?
        } else if total_offset_seconds == 0 && duration.nanos == 0 {
          write!(f, "Z")? // 'Z' for UTC
        } else {
          write!(f, "+{:02}:{:02}", hours, minutes)?
        }
      }
      Some(TimeOffset::TimeZone(tz)) => {
        // Named timezones are not usually part of the ISO 8601 string itself
        // (it usually implies fixed offset or UTC).
        // However, for debugging/clarity, we can append it in parentheses.
        write!(f, "[{}]", tz.id)?;
      }
      None => {}
    }
    Ok(())
  }
}

/// Errors that can occur during the creation, conversion or validation of a [`DateTime`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DateTimeError {
  #[error(
    "The year must be a value from 0 (to indicate a DateTime with no specific year) to 9999"
  )]
  InvalidYear,
  #[error("If the year is set to 0, month and day cannot be set to 0")]
  InvalidDate,
  #[error("Invalid month value (must be within 1 and 12)")]
  InvalidMonth,
  #[error("Invalid day value (must be within 1 and 31)")]
  InvalidDay,
  #[error("Invalid hours value (must be within 0 and 23)")]
  InvalidHours,
  #[error("Invalid minutes value (must be within 0 and 59)")]
  InvalidMinutes,
  #[error("Invalid seconds value (must be within 0 and 59)")]
  InvalidSeconds,
  #[error("Invalid nanos value (must be within 0 and 999.999.999)")]
  InvalidNanos,
  #[error(
    "DateTime has an invalid time component (e.g., hours, minutes, seconds, nanos out of range)"
  )]
  InvalidTime,
  #[error("DateTime arithmetic resulted in a time outside its representable range")]
  OutOfRange,
  #[error("DateTime conversion error: {0}")]
  ConversionError(String),
}

impl PartialOrd for TimeOffset {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match (self, other) {
      (Self::UtcOffset(a), Self::UtcOffset(b)) => a.partial_cmp(b),
      // Can't determine order without timezone information
      (Self::TimeZone(_), Self::TimeZone(_)) => None,
      (Self::UtcOffset(_), Self::TimeZone(_)) => None,
      (Self::TimeZone(_), Self::UtcOffset(_)) => None,
    }
  }
}

impl PartialOrd for DateTime {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    if !(self.is_valid() && other.is_valid()) {
      return None;
    }

    if (self.year == 0 && other.year != 0) || (self.year != 0 && other.year == 0) {
      return None;
    }

    let ord = self
      .year
      .cmp(&other.year)
      .then_with(|| self.month.cmp(&other.month))
      .then_with(|| self.day.cmp(&other.day))
      .then_with(|| self.hours.cmp(&other.hours))
      .then_with(|| self.minutes.cmp(&other.minutes))
      .then_with(|| self.seconds.cmp(&other.seconds))
      .then_with(|| self.nanos.cmp(&other.nanos));

    if ord != std::cmp::Ordering::Equal {
      return Some(ord);
    }

    self.time_offset.partial_cmp(&other.time_offset)
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
  if !(0..=9999).contains(&year) {
    return Err(DateTimeError::InvalidYear);
  }
  if !(1..=12).contains(&month) {
    return Err(DateTimeError::InvalidMonth);
  }
  if !(1..=31).contains(&day) {
    return Err(DateTimeError::InvalidDay);
  }

  if year == 0 && (day == 0 || month == 0) {
    return Err(DateTimeError::InvalidDate);
  }

  if !(0..=23).contains(&hours) {
    return Err(DateTimeError::InvalidHours);
  }
  if !(0..=59).contains(&minutes) {
    return Err(DateTimeError::InvalidMinutes);
  }
  if !(0..=59).contains(&seconds) {
    return Err(DateTimeError::InvalidSeconds);
  }
  if !(0..=999_999_999).contains(&nanos) {
    return Err(DateTimeError::InvalidNanos);
  }

  Ok(())
}

impl DateTime {
  /// Checks if this [`DateTime`] instance represents a valid date and time, and returns the related error if it does not.
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

  /// Checks if this [`DateTime`] instance represents a valid date and time.
  pub fn is_valid(&self) -> bool {
    self.validate().is_ok()
  }

  /// Returns `true` if the [`DateTime`] has a specific year (i.e., `year` is not 0).
  pub fn has_year(&self) -> bool {
    self.year != 0
  }

  /// Returns true if the [`TimeOffset`] is a UtcOffset.
  pub fn has_utc_offset(&self) -> bool {
    matches!(self.time_offset, Some(TimeOffset::UtcOffset(_)))
  }

  /// Returns true if the [`TimeOffset`] is a TimeZone.
  pub fn has_timezone(&self) -> bool {
    matches!(self.time_offset, Some(TimeOffset::TimeZone(_)))
  }

  /// Returns true if the [`TimeOffset`] is None.
  pub fn is_local(&self) -> bool {
    self.time_offset.is_none()
  }

  /// Sets the `time_offset` to a UTC offset [`Duration`], clearing any existing time zone.
  pub fn with_utc_offset(mut self, offset: Duration) -> Self {
    self.time_offset = Some(TimeOffset::UtcOffset(offset));
    self
  }

  /// Sets the `time_offset` to a [`TimeZone`], clearing any existing UTC offset.
  pub fn with_time_zone(mut self, time_zone: TimeZone) -> Self {
    self.time_offset = Some(TimeOffset::TimeZone(time_zone));
    self
  }

  #[cfg(feature = "chrono")]
  /// Converts this [`DateTime`] to [`chrono::DateTime`]<Utc>.
  /// It succeeds if the [`TimeOffset`] is a UtcOffset with 0 seconds and nanos.
  pub fn to_datetime_utc(self) -> Result<chrono::DateTime<chrono::Utc>, DateTimeError> {
    self.try_into()
  }

  #[cfg(feature = "chrono")]
  /// Converts this [`DateTime`] to [`chrono::DateTime`]<[`FixedOffset`](chrono::FixedOffset)>.
  /// It succeeds if the [`TimeOffset`] is a UtcOffset that results in an unambiguous [`FixedOffset`](chrono::FixedOffset).
  pub fn to_fixed_offset_datetime(
    self,
  ) -> Result<chrono::DateTime<chrono::FixedOffset>, DateTimeError> {
    self.try_into()
  }

  #[cfg(all(feature = "chrono", feature = "chrono-tz"))]
  /// Converts this [`DateTime`] to [`chrono::DateTime`]<[`Tz`](chrono_tz::Tz)>.
  /// It succeeds if the [`TimeOffset`] is a [`TimeZone`] that maps to a valid [`Tz`](chrono_tz::Tz) or if the [`TimeOffset`] is a UtcOffset with 0 seconds and nanos.
  pub fn to_datetime_with_tz(self) -> Result<chrono::DateTime<chrono_tz::Tz>, DateTimeError> {
    self.try_into()
  }
}

pub const UTC_OFFSET: Duration = Duration {
  seconds: 0,
  nanos: 0,
};

#[cfg(all(feature = "chrono", feature = "chrono-tz"))]
impl From<chrono_tz::Tz> for TimeZone {
  fn from(value: chrono_tz::Tz) -> Self {
    Self {
      id: value.to_string(),
      version: "".to_string(), // Version is optional according to the spec
    }
  }
}

// DateTime<Tz> conversions

#[cfg(all(feature = "chrono", feature = "chrono-tz"))]
impl From<chrono::DateTime<chrono_tz::Tz>> for DateTime {
  fn from(value: chrono::DateTime<chrono_tz::Tz>) -> Self {
    use chrono::{Datelike, Timelike};

    Self {
      year: value.year(),
      month: value.month() as i32,
      day: value.day() as i32,
      hours: value.hour() as i32,
      minutes: value.minute() as i32,
      seconds: value.second() as i32,
      nanos: value.nanosecond() as i32,
      time_offset: Some(TimeOffset::TimeZone(TimeZone {
        id: value.timezone().to_string(),
        version: "".to_string(), // Version is optional according to the spec
      })),
    }
  }
}

#[cfg(all(feature = "chrono", feature = "chrono-tz"))]
impl TryFrom<DateTime> for chrono::DateTime<chrono_tz::Tz> {
  type Error = DateTimeError;

  fn try_from(value: crate::DateTime) -> Result<Self, Self::Error> {
    use std::str::FromStr;

    use chrono::{NaiveDateTime, TimeZone};
    use chrono_tz::Tz;

    let timezone = match &value.time_offset {
      Some(TimeOffset::UtcOffset(proto_duration)) => {
        if *proto_duration == UTC_OFFSET {
          Tz::UTC
        } else {
          return Err(DateTimeError::ConversionError(
            "Cannot convert non-zero UtcOffset to a named TimeZone (Tz)".to_string(),
          ));
        }
      }
      // Case B: TimeZone (named IANA string) -> Use chrono_tz::Tz::from_str
      Some(TimeOffset::TimeZone(tz_name)) => Tz::from_str(&tz_name.id).map_err(|_| {
        DateTimeError::ConversionError(format!(
          "Unrecognized or invalid timezone name: {}",
          tz_name.id
        ))
      })?,
      None => {
        return Err(DateTimeError::ConversionError(
          "Cannot convert local DateTime to named TimeZone (Tz) without explicit offset or name"
            .to_string(),
        ));
      }
    };

    let naive_dt: NaiveDateTime = value.try_into()?;

    timezone
      .from_local_datetime(&naive_dt)
      .single()
      .ok_or(DateTimeError::ConversionError(
        "Ambiguous or invalid local time to named TimeZone (Tz) conversion".to_string(),
      ))
  }
}

// FixedOffset conversions
// From FixedOffset to DateTime is not possible because the values for the offset are not retrievable

#[cfg(feature = "chrono")]
impl TryFrom<DateTime> for chrono::DateTime<chrono::FixedOffset> {
  type Error = DateTimeError;
  fn try_from(value: DateTime) -> Result<Self, Self::Error> {
    let offset = match &value.time_offset {
      Some(TimeOffset::UtcOffset(proto_duration)) => {
        use crate::constants::NANOS_PER_SECOND;

        let total_nanos_i128 = (proto_duration.seconds as i128)
          .checked_mul(NANOS_PER_SECOND as i128)
          .ok_or(DateTimeError::ConversionError(
            "UtcOffset seconds multiplied by NANOS_PER_SECOND overflowed i128".to_string(),
          ))?
          .checked_add(proto_duration.nanos as i128)
          .ok_or(DateTimeError::ConversionError(
            "UtcOffset nanos addition overflowed i128".to_string(),
          ))?;

        let total_seconds_i128 = total_nanos_i128
          .checked_div(NANOS_PER_SECOND as i128)
          .ok_or(DateTimeError::ConversionError(
            "UtcOffset total nanoseconds division overflowed i128 (should not happen)".to_string(),
          ))?; // Division by zero not possible for NANOS_PER_SECOND

        let total_seconds_i32: i32 = total_seconds_i128.try_into().map_err(|_| {
          DateTimeError::ConversionError(
            "UtcOffset total seconds is outside of i32 range for FixedOffset".to_string(),
          )
        })?;

        chrono::FixedOffset::east_opt(total_seconds_i32).ok_or_else(|| {
          DateTimeError::ConversionError(
            "Failed to convert proto::Duration to chrono::FixedOffset due to invalid offset values"
              .to_string(),
          )
        })
      }
      Some(TimeOffset::TimeZone(_)) => Err(DateTimeError::ConversionError(
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

// NaiveDateTime conversions

#[cfg(feature = "chrono")]
impl From<chrono::NaiveDateTime> for DateTime {
  fn from(ndt: chrono::NaiveDateTime) -> Self {
    use chrono::{Datelike, Timelike};

    // NaiveDateTime has no offset, so DateTime will be local time
    // Casting is safe due to chrono's constructor API
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

    // Casting is safe after validation
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

// UTC Conversions

#[cfg(feature = "chrono")]
impl From<chrono::DateTime<chrono::Utc>> for DateTime {
  fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
    use chrono::{Datelike, Timelike};
    // Casting is safe due to chrono's constructor API
    DateTime {
      year: value.year(),
      month: value.month() as i32,
      day: value.day() as i32,
      hours: value.hour() as i32,
      minutes: value.minute() as i32,
      seconds: value.second() as i32,
      nanos: value.nanosecond() as i32,
      time_offset: Some(TimeOffset::UtcOffset(Duration::new(0, 0))),
    }
  }
}

#[cfg(feature = "chrono")]
impl TryFrom<DateTime> for chrono::DateTime<chrono::Utc> {
  type Error = DateTimeError;
  fn try_from(value: DateTime) -> Result<Self, Self::Error> {
    match &value.time_offset {
      Some(TimeOffset::UtcOffset(proto_duration)) => {
        if *proto_duration != UTC_OFFSET {
          return Err(DateTimeError::ConversionError(
            "Cannot convert DateTime to TimeZone<Utc> when the UtcOffset is not 0.".to_string(),
          ));
        }
      }
      Some(TimeOffset::TimeZone(_)) => {
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
