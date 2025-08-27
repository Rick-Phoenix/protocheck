use serde::{
  de::{self, Visitor},
  ser::Serializer,
  Deserialize, Deserializer, Serialize,
};

use crate::rpc::Code;

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

    impl<'de> Visitor<'de> for CodeVisitor {
      type Value = Code;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string representing a Code enum variant (e.g., \"OK\", \"UNKNOWN\")")
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Code::from_str_name(v).ok_or_else(|| E::custom(format!("unknown Code variant: {}", v)))
      }
    }

    deserializer.deserialize_str(CodeVisitor)
  }
}
