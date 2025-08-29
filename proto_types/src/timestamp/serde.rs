use std::fmt;

use chrono::{DateTime, Utc};
use serde::{de, ser};

use crate::Timestamp;

impl ser::Serialize for Timestamp {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    let mut ts_normalized = *self;
    ts_normalized.normalize();

    let datetime_opt =
    // Safe casting due to normalize() changing the sign to positive
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
