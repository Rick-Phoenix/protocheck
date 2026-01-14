use crate::*;

impl FieldMask {
  #[must_use]
  pub const fn new(paths: Vec<String>) -> Self {
    Self { paths }
  }

  #[must_use]
  pub const fn is_empty(&self) -> bool {
    self.paths.is_empty()
  }

  #[must_use]
  pub fn contains(&self, path: &str) -> bool {
    self.paths.iter().any(|p| p == path)
  }

  pub fn add_path(&mut self, path: &str) {
    self.paths.push(path.to_string());
  }
}

#[cfg(feature = "serde")]
mod serde_impls {
  use super::*;

  use core::fmt;

  use serde::{Deserialize, Serialize};

  use crate::FieldMask;
  impl Serialize for FieldMask {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer,
    {
      let joined_paths = self.paths.join(",");
      serializer.serialize_str(&joined_paths)
    }
  }

  impl<'de> Deserialize<'de> for FieldMask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'de>,
    {
      struct FieldMaskVisitor;

      impl serde::de::Visitor<'_> for FieldMaskVisitor {
        type Value = FieldMask;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
          formatter.write_str("a comma-separated string of field paths")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
          E: serde::de::Error,
        {
          if value.is_empty() {
            return Ok(FieldMask { paths: Vec::new() });
          }

          let paths: Vec<String> = value
            .split(",")
            .map(|s| s.trim().to_string())
            .collect();

          Ok(FieldMask { paths })
        }
      }

      deserializer.deserialize_str(FieldMaskVisitor)
    }
  }
}
