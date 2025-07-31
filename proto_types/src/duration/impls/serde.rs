use core::fmt;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::{Duration, NANOS_PER_SECOND};

impl Serialize for Duration {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut self_normalized = *self;
    self_normalized.normalize();

    let seconds: f64 =
      self_normalized.seconds as f64 + self_normalized.nanos as f64 / NANOS_PER_SECOND as f64;
    serializer.serialize_str(&format!("{:.9}s", seconds))
  }
}

impl<'de> Deserialize<'de> for Duration {
  fn deserialize<D>(deserializer: D) -> Result<Duration, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct DurationVisitor;

    impl de::Visitor<'_> for DurationVisitor {
      type Value = Duration;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A duration ending in 's'")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        if !value.ends_with('s') {
          return Err(de::Error::custom("Duration should end with 's'"));
        }

        let duration_str = &value[..value.len() - 1]; // Remove 's' from the end

        let mut parts = duration_str.split('.'); // Split seconds and fractional seconds

        let seconds: i64 = parts
          .next()
          .ok_or_else(|| de::Error::custom("Missing seconds"))?
          .parse()
          .map_err(de::Error::custom)?;

        let nanos: i32 = match parts.next() {
          Some(fraction) => {
            let mut fraction_str = fraction.to_string(); // Need to own it for modification
                                                         // Pad fraction to 9 digits (nanoseconds)
            if fraction_str.len() > 9 {
              // Handle too many fractional digits
              return Err(de::Error::custom(format!(
                "Fractional part has more than 9 digits: {}",
                fraction_str.len()
              )));
            }
            fraction_str.reserve(9 - fraction_str.len()); // Pre-allocate to avoid reallocations
            for _ in fraction_str.len()..9 {
              fraction_str.push('0');
            }

            fraction_str.parse().map_err(de::Error::custom)?
          }
          None => 0,
        };

        let mut duration = Duration { seconds, nanos };
        duration.normalize(); // Normalize after creation

        Ok(duration)
      }
    }

    deserializer.deserialize_str(DurationVisitor)
  }
}
