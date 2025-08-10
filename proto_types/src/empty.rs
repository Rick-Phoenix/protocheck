use std::fmt;

use prost::Name;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use crate::{type_url_for, Empty, PACKAGE_PREFIX};

impl From<()> for Empty {
  fn from(_: ()) -> Self {
    Empty {}
  }
}

impl Name for Empty {
  const PACKAGE: &'static str = PACKAGE_PREFIX;

  const NAME: &'static str = "Empty";

  fn type_url() -> String {
    type_url_for::<Self>()
  }
}

impl Serialize for Empty {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    // Serialize as an empty struct (which maps to an empty JSON object `{}`)
    serializer.serialize_struct("Empty", 0)?.end()
  }
}

impl<'de> Deserialize<'de> for Empty {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct EmptyVisitor;

    impl<'de> serde::de::Visitor<'de> for EmptyVisitor {
      type Value = Empty;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an empty object `{}`")
      }

      fn visit_map<A>(self, mut _map: A) -> Result<Self::Value, A::Error>
      where
        A: serde::de::MapAccess<'de>,
      {
        // Ensure there are no unexpected fields in the map
        if let Some(key) = _map.next_key::<String>()? {
          return Err(serde::de::Error::custom(format!(
            "Unexpected field in Empty message: {}",
            key
          )));
        }
        Ok(Empty {})
      }

      // Also allow deserializing from unit (`()`) if needed, though `{}` is standard for JSON
      fn visit_unit<E>(self) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Empty {})
      }
    }

    deserializer.deserialize_unit_struct("Empty", EmptyVisitor) // Expect a struct with no fields
  }
}
