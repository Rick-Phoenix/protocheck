#[cfg(feature = "serde")]
mod serde {
  use crate::*;

  use alloc::borrow::ToOwned;
  use base64::{Engine, prelude::BASE64_STANDARD};
  use prost::bytes::Bytes;
  use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::Serializer,
  };

  use crate::{BytesValue, ListValue, NullValue, Struct, Value, value::Kind};

  impl Serialize for ListValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      self.values.serialize(serializer)
    }
  }

  impl<'de> Deserialize<'de> for ListValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: de::Deserializer<'de>,
    {
      let values = <::prost::alloc::vec::Vec<Value>>::deserialize(deserializer)?;
      Ok(Self { values })
    }
  }

  impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      match self.kind {
        Some(Kind::NullValue(_)) => serializer.serialize_unit(),
        Some(Kind::NumberValue(v)) => serializer.serialize_f64(v),
        Some(Kind::StringValue(ref v)) => serializer.serialize_str(v),
        Some(Kind::BoolValue(v)) => serializer.serialize_bool(v),
        Some(Kind::StructValue(ref v)) => v.serialize(serializer),
        Some(Kind::ListValue(ref v)) => v.serialize(serializer),
        None => Err(serde::ser::Error::custom("Value must have a variant set")),
      }
    }
  }

  impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: de::Deserializer<'de>,
    {
      deserializer.deserialize_any(ValueVisitor)
    }
  }

  struct ValueVisitor;

  impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
      formatter.write_str("a JSON value (null, number, string, boolean, object, or array)")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(Value {
        kind: Some(Kind::NullValue(NullValue::NullValue as i32)),
      })
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(Value {
        kind: Some(Kind::BoolValue(v)),
      })
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(Value {
        kind: Some(Kind::NumberValue(v)),
      })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(Value {
        kind: Some(Kind::StringValue(v)),
      })
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      self.visit_string(v.to_owned())
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
      A: MapAccess<'de>,
    {
      let s = Struct::deserialize(de::value::MapAccessDeserializer::new(map))?;
      Ok(Value {
        kind: Some(Kind::StructValue(s)),
      })
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
      A: SeqAccess<'de>,
    {
      let l = ListValue::deserialize(de::value::SeqAccessDeserializer::new(seq))?;
      Ok(Value {
        kind: Some(Kind::ListValue(l)),
      })
    }
  }

  impl<'de> Deserialize<'de> for BytesValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      struct BytesValueVisitor;

      impl Visitor<'_> for BytesValueVisitor {
        type Value = BytesValue;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
          formatter.write_str("a base64 encoded string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
          E: de::Error,
        {
          BASE64_STANDARD
            .decode(v)
            .map(|value| BytesValue {
              value: Bytes::from(value),
            })
            .map_err(de::Error::custom)
        }
      }

      deserializer.deserialize_str(BytesValueVisitor)
    }
  }
}
