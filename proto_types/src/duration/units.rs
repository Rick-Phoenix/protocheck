#![allow(dead_code)]

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seconds {
  pub value: u64,
}

impl Seconds {
  pub fn format(&self) -> String {
    format!(
      "{} second{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

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

impl Minutes {
  pub fn format(&self) -> String {
    format!(
      "{} minute{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

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

impl Hours {
  pub fn format(&self) -> String {
    format!(
      "{} hour{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

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

impl Days {
  pub fn format(&self) -> String {
    format!(
      "{} day{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

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
  pub fn format(&self) -> String {
    format!(
      "{} week{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

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
  pub fn format(&self) -> String {
    format!(
      "{} month{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

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
  fn format(&self) -> String {
    format!(
      "{} year{}",
      self.value,
      if self.value != 1 { "s" } else { "" }
    )
  }

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
