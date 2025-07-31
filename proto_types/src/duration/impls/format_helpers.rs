#![allow(clippy::option_map_unit_fn)]
use crate::{Duration, DurationData};

impl Duration {
  pub fn display_full(&self) -> String {
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
      return "0 seconds".to_string();
    }

    match parts.len() {
      1 => parts.remove(0),
      2 => format!("{}{} and {}", sign, parts[0], parts[1]),
      _ => {
        let last = parts.pop().unwrap();
        format!("{}{}, and {}", sign, parts.join(", "), last)
      }
    }
  }
}
