use serde::{
  Deserialize, Deserializer, Serialize,
  de::{self, Visitor},
  ser::Serializer,
};

use crate::{Code, format};

impl Serialize for Code {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.as_str_name())
  }
}

impl<'de> Deserialize<'de> for Code {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct CodeVisitor;

    impl Visitor<'_> for CodeVisitor {
      type Value = Code;

      fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a string representing a Code enum variant (e.g., \"OK\", \"UNKNOWN\")")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Code::from_str_name(v).ok_or_else(|| E::custom(format!("unknown Code variant: {v}")))
      }
    }

    deserializer.deserialize_str(CodeVisitor)
  }
}
