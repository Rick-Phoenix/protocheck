#![allow(clippy::len_without_is_empty)]
include!("./buf.validate.rs");

use crate::protovalidate::field_path_element::Subscript;

mod violations;

use crate::{Display, String, ToString, fmt};

pub use violations::*;

impl From<usize> for Subscript {
  fn from(value: usize) -> Self {
    Self::Index(value as u64)
  }
}

impl From<i64> for Subscript {
  fn from(value: i64) -> Self {
    Self::IntKey(value)
  }
}

impl From<i32> for Subscript {
  fn from(value: i32) -> Self {
    Self::IntKey(value.into())
  }
}

impl From<u64> for Subscript {
  fn from(value: u64) -> Self {
    Self::UintKey(value)
  }
}

impl From<u32> for Subscript {
  fn from(value: u32) -> Self {
    Self::UintKey(value.into())
  }
}

impl From<bool> for Subscript {
  fn from(value: bool) -> Self {
    Self::BoolKey(value)
  }
}

impl From<String> for Subscript {
  fn from(value: String) -> Self {
    Self::StringKey(value)
  }
}

impl From<&str> for Subscript {
  fn from(value: &str) -> Self {
    Self::StringKey(value.to_string())
  }
}

impl Display for Subscript {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::BoolKey(val) => write!(f, "{val}"),
      Self::IntKey(val) => write!(f, "{val}"),
      Self::Index(val) | Self::UintKey(val) => write!(f, "{val}"),
      Self::StringKey(val) => write!(f, "{val}"),
    }
  }
}
