use std::{collections::HashMap, fmt};

use cel_interpreter::{objects::Key as CelKey, Value as CelValue};
use serde::{Deserialize, Serialize};

use crate::FieldMask;

impl FieldMask {
  pub fn new(paths: Vec<String>) -> Self {
    FieldMask { paths }
  }

  pub fn is_empty(&self) -> bool {
    self.paths.is_empty()
  }
}

impl From<FieldMask> for CelValue {
  fn from(value: FieldMask) -> Self {
    let paths = &value.paths;

    let mut cel_vals: Vec<CelValue> = Vec::new();
    for path in paths {
      cel_vals.push(CelValue::String(path.clone().into()));
    }

    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    cel_map.insert("paths".into(), CelValue::List(cel_vals.into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<&FieldMask> for CelValue {
  fn from(value: &FieldMask) -> Self {
    let paths = &value.paths;

    let mut cel_vals: Vec<CelValue> = Vec::new();
    for path in paths {
      cel_vals.push(CelValue::String(path.clone().into()));
    }

    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    cel_map.insert("paths".into(), CelValue::List(cel_vals.into()));

    CelValue::Map(cel_map.into())
  }
}

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

        let paths: Vec<String> = value.split(",").map(|s| s.trim().to_string()).collect();

        Ok(FieldMask { paths })
      }
    }

    deserializer.deserialize_str(FieldMaskVisitor)
  }
}
