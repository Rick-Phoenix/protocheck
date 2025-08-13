#![allow(dead_code)]

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seconds {
  pub value: u64,
}

impl Seconds {
  /// Returns a string displaying the amount of seconds (e.g. "1 second", "2 seconds")
  pub fn format(&self) -> String {
    format!(
      "{} second{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

  /// Returns a string with the amount of seconds, but only if the amount is more than 0.
  pub fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} second{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    ))
  }

  pub fn is_zero(&self) -> bool {
    self.value == 0
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Minutes {
  pub value: u64,
}

/// Returns a string displaying the amount of minutes (e.g. "1 minute", "2 minutes")
impl Minutes {
  pub fn format(&self) -> String {
    format!(
      "{} minute{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

  /// Returns a string with the amount of minutes, but only if the amount is more than 0.
  pub fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} minute{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    ))
  }

  pub fn is_zero(&self) -> bool {
    self.value == 0
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hours {
  pub value: u64,
}

/// Returns a string displaying the amount of hours (e.g. "1 hour", "2 hours")
impl Hours {
  pub fn format(&self) -> String {
    format!(
      "{} hour{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

  /// Returns a string with the amount of hours, but only if the amount is more than 0.
  pub fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} hour{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    ))
  }

  pub fn is_zero(&self) -> bool {
    self.value == 0
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Days {
  pub value: u64,
}

/// Returns a string displaying the amount of days (e.g. "1 day", "2 days")
impl Days {
  pub fn format(&self) -> String {
    format!(
      "{} day{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

  /// Returns a string with the amount of days, but only if the amount is more than 0.
  pub fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} day{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    ))
  }

  pub fn is_zero(&self) -> bool {
    self.value == 0
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Weeks {
  pub value: u64,
}

impl Weeks {
  /// Returns a string displaying the amount of weeks (e.g. "1 week", "2 weeks")
  pub fn format(&self) -> String {
    format!(
      "{} week{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

  /// Returns a string with the amount of weeks, but only if the amount is more than 0.
  pub fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} week{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    ))
  }

  pub fn is_zero(&self) -> bool {
    self.value == 0
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Months {
  pub value: u64,
}

impl Months {
  /// Returns a string displaying the amount of months (e.g. "1 month", "2 months")
  pub fn format(&self) -> String {
    format!(
      "{} month{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

  /// Returns a string with the amount of months, but only if the amount is more than 0.
  pub fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} month{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    ))
  }

  fn is_zero(&self) -> bool {
    self.value == 0
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Years {
  pub value: u64,
}

impl Years {
  /// Returns a string displaying the amount of years (e.g. "1 year", "2 years")
  fn format(&self) -> String {
    format!(
      "{} year{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

  /// Returns a string with the amount of years, but only if the amount is more than 0.
  fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} year{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    ))
  }

  fn is_zero(&self) -> bool {
    self.value == 0
  }
}
