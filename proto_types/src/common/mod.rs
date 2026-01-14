#![allow(clippy::doc_overindented_list_items)]
#![allow(clippy::doc_lazy_continuation)]

use core::fmt::Display;

include!("./google.type.rs");

#[cfg(feature = "serde")]
mod common_serde_impls;

#[cfg(feature = "cel")]
mod cel_common_types_impls;

/// Implementations for the google.type.LatLng message.
#[cfg(feature = "latlng")]
pub mod latlng;

/// Implementations for the google.type.Color message.
#[cfg(feature = "color")]
pub mod color;

/// Implementations for the google.type.Date message.
#[cfg(feature = "date")]
pub mod date;

/// Implementations for the google.type.DateTime message.
#[cfg(feature = "datetime")]
pub mod datetime;

/// Implementations for the google.type.Decimal message.
#[cfg(feature = "decimal")]
pub mod decimal;

/// Implementations for the google.type.Fraction message.
#[cfg(feature = "fraction")]
pub mod fraction;

/// Implementations for the google.type.Interval message.
#[cfg(feature = "interval")]
pub mod interval;

#[cfg(feature = "localized_text")]
mod localized_text;

#[cfg(feature = "money")]
pub mod money;

#[cfg(feature = "postal_address")]
mod postal_address;

/// Implementations for the google.type.TimeOfDay message.
#[cfg(feature = "timeofday")]
pub mod time_of_day;

#[cfg(feature = "phone_number")]
impl PhoneNumber {
  /// Returns a new [`PhoneNumber`] instance. Ensures that `kind` is always set, as required by the spec.
  #[must_use]
  pub const fn new(extension: crate::String, kind: phone_number::Kind) -> Self {
    Self {
      extension,
      kind: Some(kind),
    }
  }

  /// Returns false if the field `kind` is missing, which means that the instance is invalid.
  #[must_use]
  pub const fn has_kind(&self) -> bool {
    self.kind.is_some()
  }
}

impl CalendarPeriod {
  /// Checks if the value is of the `unspecified` variant.
  #[must_use]
  pub const fn is_unspecified(&self) -> bool {
    matches!(self, Self::Unspecified)
  }

  /// Checks if the calendar period is a day.
  #[must_use]
  pub const fn is_day(&self) -> bool {
    matches!(self, Self::Day)
  }

  /// Checks if the calendar period is a week.
  #[must_use]
  pub const fn is_week(&self) -> bool {
    matches!(self, Self::Week)
  }

  /// Checks if the calendar period is a fortnight.
  #[must_use]
  pub const fn is_fortnight(&self) -> bool {
    matches!(self, Self::Fortnight)
  }

  /// Checks if the calendar period is a month.
  #[must_use]
  pub const fn is_month(&self) -> bool {
    matches!(self, Self::Month)
  }

  /// Checks if the calendar period is a quarter.
  #[must_use]
  pub const fn is_quarter(&self) -> bool {
    matches!(self, Self::Quarter)
  }

  /// Checks if the calendar period is a half-year.
  #[must_use]
  pub const fn is_half(&self) -> bool {
    matches!(self, Self::Half)
  }

  /// Checks if the calendar period is a year.
  #[must_use]
  pub const fn is_year(&self) -> bool {
    matches!(self, Self::Year)
  }
}

impl DayOfWeek {
  /// Checks if the value is of the `unspecified` variant.
  #[must_use]
  pub const fn is_unspecified(&self) -> bool {
    matches!(self, Self::Unspecified)
  }

  /// Returns true if the day of the variant is Monday.
  #[must_use]
  pub const fn is_monday(&self) -> bool {
    matches!(self, Self::Monday)
  }

  /// Returns true if the day of the variant is Tuesday.
  #[must_use]
  pub const fn is_tuesday(&self) -> bool {
    matches!(self, Self::Tuesday)
  }

  /// Returns true if the day of the variant is Wednesday.
  #[must_use]
  pub const fn is_wednesday(&self) -> bool {
    matches!(self, Self::Wednesday)
  }

  /// Returns true if the day of the variant is Thursday.
  #[must_use]
  pub const fn is_thursday(&self) -> bool {
    matches!(self, Self::Thursday)
  }

  /// Returns true if the day of the variant is Friday.
  #[must_use]
  pub const fn is_friday(&self) -> bool {
    matches!(self, Self::Friday)
  }

  /// Returns true if the day of the variant is Saturday.
  #[must_use]
  pub const fn is_saturday(&self) -> bool {
    matches!(self, Self::Saturday)
  }

  /// Returns true if the day of the variant is Sunday.
  #[must_use]
  pub const fn is_sunday(&self) -> bool {
    matches!(self, Self::Sunday)
  }

  /// Returns the name of the day in title case.
  #[must_use]
  pub const fn as_title_case(&self) -> &'static str {
    match self {
      Self::Unspecified => "Unspecified",
      Self::Monday => "Monday",
      Self::Tuesday => "Tuesday",
      Self::Wednesday => "Wednesday",
      Self::Thursday => "Thursday",
      Self::Friday => "Friday",
      Self::Saturday => "Saturday",
      Self::Sunday => "Sunday",
    }
  }
}

impl Month {
  /// Checks if the value is of the `Unspecified` variant.
  #[must_use]
  pub const fn is_unspecified(&self) -> bool {
    matches!(self, Self::Unspecified)
  }

  /// Returns true if the month variant is January.
  #[must_use]
  pub const fn is_january(&self) -> bool {
    matches!(self, Self::January)
  }

  /// Returns true if the month variant is February.
  #[must_use]
  pub const fn is_february(&self) -> bool {
    matches!(self, Self::February)
  }

  /// Returns true if the month variant is March.
  #[must_use]
  pub const fn is_march(&self) -> bool {
    matches!(self, Self::March)
  }

  /// Returns true if the month variant is April.
  #[must_use]
  pub const fn is_april(&self) -> bool {
    matches!(self, Self::April)
  }

  /// Returns true if the month variant is May.
  #[must_use]
  pub const fn is_may(&self) -> bool {
    matches!(self, Self::May)
  }

  /// Returns true if the month variant is June.
  #[must_use]
  pub const fn is_june(&self) -> bool {
    matches!(self, Self::June)
  }

  /// Returns true if the month variant is July.
  #[must_use]
  pub const fn is_july(&self) -> bool {
    matches!(self, Self::July)
  }

  /// Returns true if the month variant is August.
  #[must_use]
  pub const fn is_august(&self) -> bool {
    matches!(self, Self::August)
  }

  /// Returns true if the month variant is September.
  #[must_use]
  pub const fn is_september(&self) -> bool {
    matches!(self, Self::September)
  }

  /// Returns true if the month variant is October.
  #[must_use]
  pub const fn is_october(&self) -> bool {
    matches!(self, Self::October)
  }

  /// Returns true if the month variant is November.
  #[must_use]
  pub const fn is_november(&self) -> bool {
    matches!(self, Self::November)
  }

  /// Returns true if the month variant is December.
  #[must_use]
  pub const fn is_december(&self) -> bool {
    matches!(self, Self::December)
  }

  /// Returns the name of the month in title case.
  #[must_use]
  pub const fn as_title_case(&self) -> &'static str {
    match self {
      Self::Unspecified => "Unspecified",
      Self::January => "January",
      Self::February => "February",
      Self::March => "March",
      Self::April => "April",
      Self::May => "May",
      Self::June => "June",
      Self::July => "July",
      Self::August => "August",
      Self::September => "September",
      Self::October => "October",
      Self::November => "November",
      Self::December => "December",
    }
  }
}

impl Display for DayOfWeek {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.as_title_case())
  }
}

impl Display for Month {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.as_title_case())
  }
}
