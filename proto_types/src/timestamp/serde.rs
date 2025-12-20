use std::fmt;

use serde::{de, ser};

use crate::Timestamp;

impl ser::Serialize for Timestamp {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    let mut ts_normalized = *self;
    ts_normalized.normalize();

    serializer.serialize_str(&self.to_string())
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
          .parse::<Timestamp>()
          .map_err(|e| de::Error::custom(format!("Invalid timestamp string format: {e}")))?;

        let prost_timestamp = Timestamp {
          seconds: datetime.seconds,
          nanos: datetime.nanos,
        };

        Ok(prost_timestamp)
      }
    }

    deserializer.deserialize_str(TimestampVisitor)
  }
}
