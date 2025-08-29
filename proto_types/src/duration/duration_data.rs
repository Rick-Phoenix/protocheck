use super::data::{Days, Hours, Minutes, Months, Seconds, Weeks, Years};
use crate::{constants::*, Duration};

/// The data for a given Duration
#[derive(Debug, Default, Clone)]
pub struct DurationData {
  pub years: Years,
  pub months: Months,
  pub weeks: Weeks,
  pub days: Days,
  pub hours: Hours,
  pub minutes: Minutes,
  pub seconds: Seconds,
  pub is_negative: bool,
}

impl Duration {
  /// Creates a DurationData instance.
  pub fn get_data(&self) -> DurationData {
    let mut total_seconds = self.seconds.unsigned_abs();
    let years = Years {
      value: total_seconds / SECONDS_PER_YEAR_AVG,
    };
    total_seconds %= SECONDS_PER_YEAR_AVG;

    let months = Months {
      value: total_seconds / SECONDS_PER_MONTH_AVG,
    };
    total_seconds %= SECONDS_PER_MONTH_AVG;

    let weeks = Weeks {
      value: total_seconds / SECONDS_PER_WEEK,
    };
    total_seconds %= SECONDS_PER_WEEK;

    let days = Days {
      value: total_seconds / SECONDS_PER_DAY,
    };
    total_seconds %= SECONDS_PER_DAY;

    let hours = Hours {
      value: total_seconds / SECONDS_PER_HOUR,
    };
    total_seconds %= SECONDS_PER_HOUR;

    let minutes = Minutes {
      value: total_seconds / SECONDS_PER_MINUTE,
    };
    let seconds = Seconds {
      value: total_seconds % SECONDS_PER_MINUTE,
    };

    DurationData {
      years,
      months,
      weeks,
      days,
      hours,
      minutes,
      seconds,
      is_negative: self.is_negative(),
    }
  }
}
