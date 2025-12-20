/// A struct representing seconds. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seconds {
  pub value: u64,
}

impl Seconds {
  /// Returns a string displaying the amount of seconds (e.g. "1 second", "2 seconds")
  #[must_use] 
  pub fn format(&self) -> String {
    format!(
      "{} second{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    )
  }

  /// Returns a string with the amount of seconds, but only if the amount is more than 0.
  #[must_use] 
  pub fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} second{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    ))
  }

  const fn is_zero(&self) -> bool {
    self.value == 0
  }
}

/// A struct representing minutes. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Minutes {
  pub value: u64,
}

/// Returns a string displaying the amount of minutes (e.g. "1 minute", "2 minutes")
impl Minutes {
  #[must_use] 
  pub fn format(&self) -> String {
    format!(
      "{} minute{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    )
  }

  /// Returns a string with the amount of minutes, but only if the amount is more than 0.
  #[must_use] 
  pub fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} minute{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    ))
  }

  const fn is_zero(&self) -> bool {
    self.value == 0
  }
}

/// A struct representing hours. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hours {
  pub value: u64,
}

/// Returns a string displaying the amount of hours (e.g. "1 hour", "2 hours")
impl Hours {
  #[must_use] 
  pub fn format(&self) -> String {
    format!(
      "{} hour{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    )
  }

  /// Returns a string with the amount of hours, but only if the amount is more than 0.
  #[must_use] 
  pub fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} hour{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    ))
  }

  const fn is_zero(&self) -> bool {
    self.value == 0
  }
}

/// A struct representing days. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Days {
  pub value: u64,
}

/// Returns a string displaying the amount of days (e.g. "1 day", "2 days")
impl Days {
  #[must_use] 
  pub fn format(&self) -> String {
    format!(
      "{} day{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    )
  }

  /// Returns a string with the amount of days, but only if the amount is more than 0.
  #[must_use] 
  pub fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} day{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    ))
  }

  const fn is_zero(&self) -> bool {
    self.value == 0
  }
}

/// A struct representing weeks. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Weeks {
  pub value: u64,
}

impl Weeks {
  /// Returns a string displaying the amount of weeks (e.g. "1 week", "2 weeks")
  #[must_use] 
  pub fn format(&self) -> String {
    format!(
      "{} week{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    )
  }

  /// Returns a string with the amount of weeks, but only if the amount is more than 0.
  #[must_use] 
  pub fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} week{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    ))
  }

  /// Returns `true` if the value is zero.
  const fn is_zero(&self) -> bool {
    self.value == 0
  }
}

/// A struct representing months. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Months {
  pub value: u64,
}

impl Months {
  /// Returns a string displaying the amount of months (e.g. "1 month", "2 months")
  #[must_use] 
  pub fn format(&self) -> String {
    format!(
      "{} month{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    )
  }

  /// Returns a string with the amount of months, but only if the amount is more than 0.
  #[must_use] 
  pub fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} month{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    ))
  }

  const fn is_zero(&self) -> bool {
    self.value == 0
  }
}

/// A struct representing years. Wraps the value and provides extra formatting methods.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Years {
  pub value: u64,
}

impl Years {
  /// Returns a string displaying the amount of years (e.g. "1 year", "2 years")
  #[must_use] 
  pub fn format(&self) -> String {
    format!(
      "{} year{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    )
  }

  /// Returns a string with the amount of years, but only if the amount is more than 0.
  #[must_use] 
  pub fn format_if_nonzero(&self) -> Option<String> {
    if self.is_zero() {
      return None;
    }
    Some(format!(
      "{} year{}",
      self.value,
      if self.value == 1 { "" } else { "s" }
    ))
  }

  const fn is_zero(&self) -> bool {
    self.value == 0
  }
}
