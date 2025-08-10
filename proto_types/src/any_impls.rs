#[cfg(feature = "serde")]
mod serde {
  use std::fmt;

  use base64::{prelude::BASE64_STANDARD, Engine};
  use serde::{de, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

  use crate::Any;

  impl Serialize for Any {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let mut state = serializer.serialize_struct("Any", 2)?;
      state.serialize_field("@type", &self.type_url)?;
      state.serialize_field("value", &BASE64_STANDARD.encode(&self.value))?;
      state.end()
    }
  }

  impl<'de> Deserialize<'de> for Any {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      // Define a visitor to expect a map (JSON object)
      struct AnyVisitor;

      impl<'de> de::Visitor<'de> for AnyVisitor {
        type Value = Any;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
          formatter.write_str(
            "struct Any with fields `@type` (string) and `value` (base64-encoded string)",
          )
        }

        fn visit_map<V>(self, mut map: V) -> Result<Any, V::Error>
        where
          V: de::MapAccess<'de>,
        {
          let mut type_url: Option<String> = None;
          let mut value_base64: Option<String> = None;

          // Loop through fields, expecting "@type" and "value"
          while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
              "@type" => {
                if type_url.is_some() {
                  return Err(de::Error::duplicate_field("@type"));
                }
                type_url = Some(map.next_value()?);
              }
              "value" => {
                if value_base64.is_some() {
                  return Err(de::Error::duplicate_field("value"));
                }
                value_base64 = Some(map.next_value()?);
              }
              _ => {
                // Ignore any other fields
                let _ = map.next_value::<de::IgnoredAny>()?;
              }
            }
          }

          // Check that required fields were present
          let type_url = type_url.ok_or_else(|| de::Error::missing_field("@type"))?;
          let value_base64 = value_base64.ok_or_else(|| de::Error::missing_field("value"))?;

          // Decode base64 value
          let value = BASE64_STANDARD
            .decode(&value_base64)
            .map_err(de::Error::custom)?;

          Ok(Any { type_url, value })
        }

        // If the JSON is not an object (e.g., if it was an unwrapped WKT like a string),
        // this minimal implementation will just return an error.
      }

      deserializer.deserialize_map(AnyVisitor) // Instruct serde to expect a map/object
    }
  }
}
