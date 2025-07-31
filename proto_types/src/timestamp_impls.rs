use std::{cmp::Ordering, fmt};

use chrono::{DateTime, Utc};
use quote::{quote, ToTokens};
use serde::{de, ser};

use crate::{Timestamp, TokenStream2};

impl ToTokens for Timestamp {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let seconds = self.seconds;
    let nanos = self.nanos;

    tokens.extend(quote! {
      protocheck::types::Timestamp {
        seconds: #seconds,
        nanos: #nanos,
      }
    });
  }
}

impl ser::Serialize for Timestamp {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    let mut ts_normalized = *self;
    ts_normalized.normalize();

    let datetime_opt =
      DateTime::<Utc>::from_timestamp(ts_normalized.seconds, ts_normalized.nanos as u32);

    if let Some(datetime) = datetime_opt {
      let rfc3339_string = datetime.to_rfc3339();
      serializer.serialize_str(&rfc3339_string)
    } else {
      Err(ser::Error::custom(format!(
        "Invalid timestamp during serialization: seconds={}, nanos={}",
        ts_normalized.seconds, ts_normalized.nanos
      )))
    }
  }
}

impl<'de> de::Deserialize<'de> for Timestamp {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    struct TimestampVisitor;

    impl de::Visitor<'_> for TimestampVisitor {
      type Value = Timestamp;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an RFC 3339 formatted timestamp string")
      }

      fn visit_str<E>(self, value: &str) -> Result<Timestamp, E>
      where
        E: de::Error,
      {
        let datetime = value
          .parse::<DateTime<Utc>>()
          .map_err(|e| de::Error::custom(format!("Invalid timestamp string format: {}", e)))?;

        let prost_timestamp = Timestamp {
          seconds: datetime.timestamp(),
          nanos: datetime.timestamp_subsec_nanos() as i32,
        };

        Ok(prost_timestamp)
      }
    }

    deserializer.deserialize_str(TimestampVisitor)
  }
}

impl PartialOrd for Timestamp {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Timestamp {
  fn cmp(&self, other: &Self) -> Ordering {
    let mut self_ts_norm = *self;
    self_ts_norm.normalize();
    let self_dt = DateTime::<Utc>::from_timestamp(self_ts_norm.seconds, self_ts_norm.nanos as u32)
      .expect("Invalid Timestamp in Ord comparison for self");

    let mut other_ts_norm = *other;
    other_ts_norm.normalize();
    let other_dt =
      DateTime::<Utc>::from_timestamp(other_ts_norm.seconds, other_ts_norm.nanos as u32)
        .expect("Invalid Timestamp in Ord comparison for other");

    self_dt.cmp(&other_dt)
  }
}
