#![allow(clippy::option_map_unit_fn)]
use std::fmt::Display;

use super::data::DurationData;
use crate::Duration;

impl Display for Duration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let DurationData {
      months,
      days,
      hours,
      minutes,
      seconds,
      is_negative,
      ..
    } = self.get_data();

    let mut parts = Vec::new();

    let sign = if is_negative { "- " } else { "" };

    months.format_if_nonzero().map(|p| parts.push(p));
    days.format_if_nonzero().map(|p| parts.push(p));
    hours.format_if_nonzero().map(|p| parts.push(p));
    minutes.format_if_nonzero().map(|p| parts.push(p));
    seconds.format_if_nonzero().map(|p| parts.push(p));

    if parts.is_empty() {
      write!(f, "0 seconds")
    } else {
      let formatted_string = match parts.len() {
        1 => parts.remove(0),
        2 => format!("{}{} and {}", sign, parts[0], parts[1]),
        _ => {
          let last = parts.pop().unwrap();
          format!("{}{} and {}", sign, parts.join(" "), last)
        }
      };

      write!(f, "{}", formatted_string)
    }
  }
}
