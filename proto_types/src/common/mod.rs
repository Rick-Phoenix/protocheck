#![allow(clippy::doc_overindented_list_items)]
#![allow(clippy::doc_lazy_continuation)]
include!("./google.type.rs");

#[cfg(feature = "serde")]
mod common_serde_impls;

#[cfg(feature = "cel")]
mod cel_common_types_impls;

pub mod color;
pub mod date;
pub mod datetime;
pub mod decimal;
pub mod fraction;
pub mod interval;
pub mod localized_text;
pub mod money;
pub mod postal_address;
pub mod time_of_day;

impl CalendarPeriod {
  /// Checks if the value is of the `unspecified` variant.
  pub fn is_unspecified(&self) -> bool {
    matches!(self, Self::Unspecified)
  }

  /// Checks if the calendar period is a day.
  pub fn is_day(&self) -> bool {
    matches!(self, CalendarPeriod::Day)
  }

  /// Checks if the calendar period is a week.
  pub fn is_week(&self) -> bool {
    matches!(self, CalendarPeriod::Week)
  }

  /// Checks if the calendar period is a fortnight.
  pub fn is_fortnight(&self) -> bool {
    matches!(self, CalendarPeriod::Fortnight)
  }

  /// Checks if the calendar period is a month.
  pub fn is_month(&self) -> bool {
    matches!(self, CalendarPeriod::Month)
  }

  /// Checks if the calendar period is a quarter.
  pub fn is_quarter(&self) -> bool {
    matches!(self, CalendarPeriod::Quarter)
  }

  /// Checks if the calendar period is a half-year.
  pub fn is_half(&self) -> bool {
    matches!(self, CalendarPeriod::Half)
  }

  /// Checks if the calendar period is a year.
  pub fn is_year(&self) -> bool {
    matches!(self, CalendarPeriod::Year)
  }
}

impl DayOfWeek {
  /// Checks if the value is of the `unspecified` variant.
  pub fn is_unspecified(&self) -> bool {
    matches!(self, Self::Unspecified)
  }

  /// Returns true if the day of the variant is Monday.
  pub fn is_monday(&self) -> bool {
    matches!(self, Self::Monday)
  }

  /// Returns true if the day of the variant is Tuesday.
  pub fn is_tuesday(&self) -> bool {
    matches!(self, Self::Tuesday)
  }

  /// Returns true if the day of the variant is Wednesday.
  pub fn is_wednesday(&self) -> bool {
    matches!(self, Self::Wednesday)
  }

  /// Returns true if the day of the variant is Thursday.
  pub fn is_thursday(&self) -> bool {
    matches!(self, Self::Thursday)
  }

  /// Returns true if the day of the variant is Friday.
  pub fn is_friday(&self) -> bool {
    matches!(self, Self::Friday)
  }

  /// Returns true if the day of the variant is Saturday.
  pub fn is_saturday(&self) -> bool {
    matches!(self, Self::Saturday)
  }

  /// Returns true if the day of the variant is Sunday.
  pub fn is_sunday(&self) -> bool {
    matches!(self, Self::Sunday)
  }

  /// Returns the name of the day in title case.
  pub fn as_title_case(&self) -> &str {
    match self {
      DayOfWeek::Unspecified => "Unspecified",
      DayOfWeek::Monday => "Monday",
      DayOfWeek::Tuesday => "Tuesday",
      DayOfWeek::Wednesday => "Wednesday",
      DayOfWeek::Thursday => "Thursday",
      DayOfWeek::Friday => "Friday",
      DayOfWeek::Saturday => "Saturday",
      DayOfWeek::Sunday => "Sunday",
    }
  }
}

impl PhoneNumber {
  /// Returns false if the field `kind` is missing.
  pub fn has_kind(&self) -> bool {
    self.kind.is_some()
  }
}

impl Month {
  /// Checks if the value is of the `Unspecified` variant.
  pub fn is_unspecified(&self) -> bool {
    matches!(self, Self::Unspecified)
  }

  /// Returns true if the month variant is January.
  pub fn is_january(&self) -> bool {
    matches!(self, Self::January)
  }

  /// Returns true if the month variant is February.
  pub fn is_february(&self) -> bool {
    matches!(self, Self::February)
  }

  /// Returns true if the month variant is March.
  pub fn is_march(&self) -> bool {
    matches!(self, Self::March)
  }

  /// Returns true if the month variant is April.
  pub fn is_april(&self) -> bool {
    matches!(self, Self::April)
  }

  /// Returns true if the month variant is May.
  pub fn is_may(&self) -> bool {
    matches!(self, Self::May)
  }

  /// Returns true if the month variant is June.
  pub fn is_june(&self) -> bool {
    matches!(self, Self::June)
  }

  /// Returns true if the month variant is July.
  pub fn is_july(&self) -> bool {
    matches!(self, Self::July)
  }

  /// Returns true if the month variant is August.
  pub fn is_august(&self) -> bool {
    matches!(self, Self::August)
  }

  /// Returns true if the month variant is September.
  pub fn is_september(&self) -> bool {
    matches!(self, Self::September)
  }

  /// Returns true if the month variant is October.
  pub fn is_october(&self) -> bool {
    matches!(self, Self::October)
  }

  /// Returns true if the month variant is November.
  pub fn is_november(&self) -> bool {
    matches!(self, Self::November)
  }

  /// Returns true if the month variant is December.
  pub fn is_december(&self) -> bool {
    matches!(self, Self::December)
  }

  /// Returns the name of the month in title case.
  pub fn as_title_case(&self) -> &str {
    match self {
      Month::Unspecified => "Unspecified",
      Month::January => "January",
      Month::February => "February",
      Month::March => "March",
      Month::April => "April",
      Month::May => "May",
      Month::June => "June",
      Month::July => "July",
      Month::August => "August",
      Month::September => "September",
      Month::October => "October",
      Month::November => "November",
      Month::December => "December",
    }
  }
}
