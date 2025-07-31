use std::fmt;

use prost::Message;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Message)]
pub struct Empty {}

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
      type Value = Empty; // The type this visitor will produce

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an empty object `{}`")
      }

      // This method handles deserializing from a map (JSON object)
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
        Ok(Empty {}) // Return an empty instance
      }

      // Also allow deserializing from unit (`()`) if needed, though `{}` is standard for JSON
      fn visit_unit<E>(self) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Empty {})
      }
    }

    deserializer.deserialize_struct("Empty", &[], EmptyVisitor) // Expect a struct with no fields
  }
}
