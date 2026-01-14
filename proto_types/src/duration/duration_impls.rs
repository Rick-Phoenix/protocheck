use crate::Duration;

impl core::cmp::PartialOrd for Duration {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl core::cmp::Ord for Duration {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    (self.seconds, self.nanos).cmp(&(other.seconds, other.nanos))
  }
}

#[cfg(feature = "serde")]
mod serde {
  use core::fmt;

  use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

  use crate::{Duration, ToString, format};

  impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let self_normalized = self.normalized();

      let seconds = self_normalized.seconds;
      let nanos = self_normalized.nanos;

      let formatted_string = if nanos == 0 {
        // If nanos are zero, just "Xs"
        format!("{seconds}s")
      } else {
        let fractional_seconds_str = format!("{nanos:09}");

        let trimmed_fractional_seconds = fractional_seconds_str.trim_end_matches('0');

        format!("{seconds}.{trimmed_fractional_seconds}s")
      };

      serializer.serialize_str(&formatted_string)
    }
  }

  impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
}
