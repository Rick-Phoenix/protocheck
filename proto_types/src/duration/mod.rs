use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

mod impls;
mod units;

pub use impls::DurationError;
pub use units::*;

use crate::Duration;

const SECONDS_PER_MINUTE: u64 = 60;
const SECONDS_PER_HOUR: u64 = 3600;
const SECONDS_PER_DAY: u64 = 86400;
const SECONDS_PER_WEEK: u64 = 604800;
const SECONDS_PER_MONTH_AVG: u64 = 2_629_746;
const SECONDS_PER_YEAR_AVG: u64 = 31_556_952;

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
  pub fn is_negative(&self) -> bool {
    self.normalized().seconds < 0
  }

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

impl ToTokens for Duration {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let seconds = self.seconds;
    let nanos = self.nanos;

    tokens.extend(quote! {
      protocheck::types::Duration {
        seconds: #seconds,
        nanos: #nanos,
      }
    });
  }
}

impl std::cmp::PartialOrd for Duration {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl std::cmp::Ord for Duration {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    let mut self_normalized = *self;
    self_normalized.normalize();
    let self_chrono_duration =
      chrono::Duration::new(self_normalized.seconds, self_normalized.nanos as u32);

    let mut other_normalized = *other;
    other_normalized.normalize();
    let other_chrono_duration =
      chrono::Duration::new(other_normalized.seconds, other_normalized.nanos as u32);

    self_chrono_duration.cmp(&other_chrono_duration)
  }
}
