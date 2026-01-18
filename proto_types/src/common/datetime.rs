use core::fmt::{Display, Formatter};

use thiserror::Error;

use crate::{
  Duration, String,
  common::{DateTime, TimeZone, date_time::TimeOffset},
};

impl Display for TimeZone {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.id)
  }
}

impl Display for DateTime {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
          write!(f, "-{hours:02}:{minutes:02}")?
        } else if total_offset_seconds == 0 && duration.nanos == 0 {
          write!(f, "Z")? // 'Z' for UTC
        } else {
          write!(f, "+{hours:02}:{minutes:02}")?
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
  #[error("The year must be a value from 0 (to indicate a DateTime with no specific year) to 9999")]
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
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    match (self, other) {
      (Self::UtcOffset(a), Self::UtcOffset(b)) => a.partial_cmp(b),
      // Can't determine order without timezone information
      (Self::TimeZone(_) | Self::UtcOffset(_), Self::TimeZone(_))
      | (Self::TimeZone(_), Self::UtcOffset(_)) => None,
    }
  }
}

impl PartialOrd for DateTime {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
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

    if ord != core::cmp::Ordering::Equal {
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
  #[inline]
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

  #[must_use]
  #[inline]
  /// Checks if this [`DateTime`] instance represents a valid date and time.
  pub fn is_valid(&self) -> bool {
    self.validate().is_ok()
  }

  #[must_use]
  #[inline]
  /// Returns `true` if the [`DateTime`] has a specific year (i.e., `year` is not 0).
  pub const fn has_year(&self) -> bool {
    self.year != 0
  }

  /// Returns true if the [`TimeOffset`] is a UtcOffset.
  #[must_use]
  #[inline]
  pub const fn has_utc_offset(&self) -> bool {
    matches!(self.time_offset, Some(TimeOffset::UtcOffset(_)))
  }

  /// Returns true if the [`TimeOffset`] is a TimeZone.
  #[must_use]
  #[inline]
  pub const fn has_timezone(&self) -> bool {
    matches!(self.time_offset, Some(TimeOffset::TimeZone(_)))
  }

  /// Returns true if the [`TimeOffset`] is None.
  #[must_use]
  #[inline]
  pub const fn is_local(&self) -> bool {
    self.time_offset.is_none()
  }

  /// Sets the `time_offset` to a UTC offset [`Duration`], clearing any existing time zone.
  #[must_use]
  #[inline]
  pub fn with_utc_offset(mut self, offset: Duration) -> Self {
    self.time_offset = Some(TimeOffset::UtcOffset(offset));
    self
  }

  /// Sets the `time_offset` to a [`TimeZone`], clearing any existing UTC offset.
  #[must_use]
  #[inline]
  pub fn with_time_zone(mut self, time_zone: TimeZone) -> Self {
    self.time_offset = Some(TimeOffset::TimeZone(time_zone));
    self
  }
}

pub const UTC_OFFSET: Duration = Duration {
  seconds: 0,
  nanos: 0,
};

#[cfg(feature = "chrono")]
mod chrono {
  use chrono::Utc;

  use super::{DateTime, DateTimeError};
  use crate::{Duration, String, ToString, date_time::TimeOffset, datetime::UTC_OFFSET, format};

  impl DateTime {
    /// Returns the current [`DateTime`] with Utc offset.
    #[must_use]
    #[inline]
    pub fn now_utc() -> Self {
      Utc::now().into()
    }

    #[inline]
    /// Converts this [`DateTime`] to [`chrono::DateTime`] Utc.
    /// It succeeds if the [`TimeOffset`] is a UtcOffset with 0 seconds and nanos.
    pub fn to_datetime_utc(self) -> Result<chrono::DateTime<chrono::Utc>, DateTimeError> {
      self.try_into()
    }

    #[inline]
    /// Converts this [`DateTime`] to [`chrono::DateTime`]<[`FixedOffset`](chrono::FixedOffset)>.
    /// It succeeds if the [`TimeOffset`] is a UtcOffset that results in an unambiguous [`FixedOffset`](chrono::FixedOffset).
    pub fn to_fixed_offset_datetime(
      self,
    ) -> Result<chrono::DateTime<chrono::FixedOffset>, DateTimeError> {
      self.try_into()
    }

    #[inline]
    #[cfg(feature = "chrono-tz")]
    /// Converts this [`DateTime`] to [`chrono::DateTime`]<[`Tz`](chrono_tz::Tz)>.
    /// It succeeds if the [`TimeOffset`] is a [`TimeZone`] that maps to a valid [`Tz`](chrono_tz::Tz) or if the [`TimeOffset`] is a UtcOffset with 0 seconds and nanos.
    pub fn to_datetime_with_tz(self) -> Result<chrono::DateTime<chrono_tz::Tz>, DateTimeError> {
      self.try_into()
    }
  }

  // FixedOffset conversions
  // From FixedOffset to DateTime is not possible because the values for the offset are not retrievable

  impl TryFrom<DateTime> for chrono::DateTime<chrono::FixedOffset> {
    type Error = DateTimeError;
    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
      use crate::date_time::TimeOffset;

      let offset = match &value.time_offset {
        Some(TimeOffset::UtcOffset(proto_duration)) => {
          use crate::constants::NANOS_PER_SECOND;

          let total_nanos_i128 = i128::from(proto_duration.seconds)
            .checked_mul(i128::from(NANOS_PER_SECOND))
            .ok_or(DateTimeError::ConversionError(
              "UtcOffset seconds multiplied by NANOS_PER_SECOND overflowed i128".to_string(),
            ))?
            .checked_add(i128::from(proto_duration.nanos))
            .ok_or(DateTimeError::ConversionError(
              "UtcOffset nanos addition overflowed i128".to_string(),
            ))?;

          let total_seconds_i128 = total_nanos_i128
            .checked_div(i128::from(NANOS_PER_SECOND))
            .ok_or(DateTimeError::ConversionError(
              "UtcOffset total nanoseconds division overflowed i128 (should not happen)"
                .to_string(),
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

  impl From<chrono::NaiveDateTime> for DateTime {
    #[inline]
    fn from(ndt: chrono::NaiveDateTime) -> Self {
      use chrono::{Datelike, Timelike};

      // NaiveDateTime has no offset, so DateTime will be local time
      // Casting is safe due to chrono's constructor API
      Self {
        year: ndt.year(),
        month: ndt.month().cast_signed(),
        day: ndt.day().cast_signed(),
        hours: ndt.hour().cast_signed(),
        minutes: ndt.minute().cast_signed(),
        seconds: ndt.second().cast_signed(),
        nanos: ndt.nanosecond().cast_signed(),
        time_offset: None,
      }
    }
  }

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
      let date =
        chrono::NaiveDate::from_ymd_opt(dt.year, dt.month.cast_unsigned(), dt.day.cast_unsigned())
          .ok_or(DateTimeError::InvalidDate)?;
      let time = chrono::NaiveTime::from_hms_nano_opt(
        dt.hours.cast_unsigned(),
        dt.minutes.cast_unsigned(),
        dt.seconds.cast_unsigned(),
        dt.nanos.cast_unsigned(),
      )
      .ok_or(DateTimeError::InvalidTime)?;

      Ok(Self::new(date, time))
    }
  }

  // UTC Conversions

  impl From<chrono::DateTime<chrono::Utc>> for DateTime {
    #[inline]
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
      use chrono::{Datelike, Timelike};

      use crate::date_time::TimeOffset;
      // Casting is safe due to chrono's constructor API
      Self {
        year: value.year(),
        month: value.month().cast_signed(),
        day: value.day().cast_signed(),
        hours: value.hour().cast_signed(),
        minutes: value.minute().cast_signed(),
        seconds: value.second().cast_signed(),
        nanos: value.nanosecond().cast_signed(),
        time_offset: Some(TimeOffset::UtcOffset(Duration::new(0, 0))),
      }
    }
  }

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
        Some(TimeOffset::TimeZone(_)) | None => {
          return Err(DateTimeError::ConversionError(
            "Cannot convert DateTime to TimeZone<Utc> when a UtcOffset is not set.".to_string(),
          ));
        }
      };

      let naive_dt: chrono::NaiveDateTime = value.try_into()?;

      Ok(naive_dt.and_utc())
    }
  }

  #[cfg(feature = "chrono-tz")]
  impl From<chrono_tz::Tz> for super::TimeZone {
    fn from(value: chrono_tz::Tz) -> Self {
      Self {
        id: value.to_string(),
        version: String::new(), // Version is optional according to the spec
      }
    }
  }

  // DateTime<Tz> conversions

  #[cfg(feature = "chrono-tz")]
  impl From<chrono::DateTime<chrono_tz::Tz>> for DateTime {
    fn from(value: chrono::DateTime<chrono_tz::Tz>) -> Self {
      use chrono::{Datelike, Timelike};

      Self {
        year: value.year(),
        month: value.month().cast_signed(),
        day: value.day().cast_signed(),
        hours: value.hour().cast_signed(),
        minutes: value.minute().cast_signed(),
        seconds: value.second().cast_signed(),
        nanos: value.nanosecond().cast_signed(),
        time_offset: Some(TimeOffset::TimeZone(super::TimeZone {
          id: value.timezone().to_string(),
          version: String::new(), // Version is optional according to the spec
        })),
      }
    }
  }

  #[cfg(feature = "chrono-tz")]
  impl TryFrom<DateTime> for chrono::DateTime<chrono_tz::Tz> {
    type Error = DateTimeError;

    fn try_from(value: crate::DateTime) -> Result<Self, Self::Error> {
      use core::str::FromStr;

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
}
