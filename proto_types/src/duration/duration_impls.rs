use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::Duration;

impl Duration {
  pub fn new(seconds: i64, nanos: i32) -> Self {
    let mut instance = Duration { seconds, nanos };
    instance.normalize();
    instance
  }
}

impl ToTokens for Duration {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let seconds = self.seconds;
    let nanos = self.nanos;

    tokens.extend(quote! {
      protocheck::types::Duration {
        seconds: #seconds,
        nanos: #nanos,
      }
    });
  }
}

impl std::cmp::PartialOrd for Duration {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl std::cmp::Ord for Duration {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    let mut self_normalized = *self;
    self_normalized.normalize();
    let self_chrono_duration =
      chrono::Duration::new(self_normalized.seconds, self_normalized.nanos as u32);

    let mut other_normalized = *other;
    other_normalized.normalize();
    let other_chrono_duration =
      chrono::Duration::new(other_normalized.seconds, other_normalized.nanos as u32);

    self_chrono_duration.cmp(&other_chrono_duration)
  }
}

#[cfg(feature = "serde")]
mod serde {
  use core::fmt;

  use chrono::Duration as ChronoDuration;
  use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};

  use crate::Duration;
  impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let self_normalized = self.normalized();

      let chrono_dur: ChronoDuration = self_normalized.try_into().map_err(|e| {
        ser::Error::custom(format!(
          "Failed to convert duration for serialization: {}",
          e
        ))
      })?;

      let seconds = chrono_dur.num_seconds();
      let nanos = chrono_dur.subsec_nanos();

      let formatted_string = if nanos == 0 {
        // If nanos are zero, just "Xs"
        format!("{}s", seconds)
      } else {
        let fractional_seconds_str = format!("{:09}", nanos);

        let trimmed_fractional_seconds = fractional_seconds_str.trim_end_matches('0');

        format!("{}.{}s", seconds, trimmed_fractional_seconds)
      };

      serializer.serialize_str(&formatted_string)
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
}
