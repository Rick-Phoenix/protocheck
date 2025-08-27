#[cfg(feature = "serde")]
mod serde {
  use base64::{prelude::BASE64_STANDARD, Engine};
  use prost::bytes::Bytes;
  use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::Serializer,
    Deserialize, Deserializer, Serialize,
  };

  use crate::{value::Kind, BytesValue, ListValue, NullValue, Struct, Value};

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
      Ok(ListValue { values })
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

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
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

      impl<'de> Visitor<'de> for BytesValueVisitor {
        type Value = BytesValue;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
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

#[cfg(feature = "totokens")]
mod totokens {
  use proc_macro2::TokenStream;
  use quote::{quote, ToTokens};

  use crate::field_descriptor_proto::Type as ProtoType;

  impl ToTokens for ProtoType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let path = quote! { ::protocheck::types::field_descriptor_proto::Type };

      match self {
        ProtoType::Double => tokens.extend(quote! { #path::Double }),
        ProtoType::Float => tokens.extend(quote! { #path::Float }),
        ProtoType::Int64 => tokens.extend(quote! { #path::Int64 }),
        ProtoType::Uint64 => tokens.extend(quote! { #path::Uint64 }),
        ProtoType::Int32 => tokens.extend(quote! { #path::Int32 }),
        ProtoType::Fixed64 => tokens.extend(quote! { #path::Fixed64 }),
        ProtoType::Fixed32 => tokens.extend(quote! { #path::Fixed32 }),
        ProtoType::Bool => tokens.extend(quote! { #path::Bool }),
        ProtoType::String => tokens.extend(quote! { #path::String }),
        ProtoType::Group => tokens.extend(quote! { #path::Group }),
        ProtoType::Message => tokens.extend(quote! { #path::Message }),
        ProtoType::Bytes => tokens.extend(quote! { #path::Bytes }),
        ProtoType::Uint32 => tokens.extend(quote! { #path::Uint32 }),
        ProtoType::Enum => tokens.extend(quote! { #path::Enum }),
        ProtoType::Sfixed32 => tokens.extend(quote! { #path::Sfixed32 }),
        ProtoType::Sfixed64 => tokens.extend(quote! { #path::Sfixed64 }),
        ProtoType::Sint32 => tokens.extend(quote! { #path::Sint32 }),
        ProtoType::Sint64 => tokens.extend(quote! { #path::Sint64 }),
      }
    }
  }
}
