#[cfg(feature = "chrono")]
mod chrono {
  use std::convert::{From, TryFrom};

  use chrono::{DateTime, FixedOffset, Utc};

  use crate::{timestamp::TimestampError, Timestamp};
  impl From<DateTime<Utc>> for Timestamp {
    fn from(datetime: DateTime<Utc>) -> Self {
      let mut ts = Timestamp {
        seconds: datetime.timestamp(),
        // Safe casting as this value is limited by chrono
        nanos: datetime.timestamp_subsec_nanos() as i32,
      };
      ts.normalize();
      ts
    }
  }

  impl TryFrom<Timestamp> for DateTime<Utc> {
    type Error = TimestampError;

    fn try_from(mut timestamp: Timestamp) -> Result<Self, Self::Error> {
      timestamp.normalize();

      DateTime::<Utc>::from_timestamp(timestamp.seconds, timestamp.nanos as u32)
        .ok_or(TimestampError::OutOfSystemRange(timestamp))
    }
  }

  impl TryFrom<Timestamp> for DateTime<FixedOffset> {
    type Error = TimestampError;

    fn try_from(mut timestamp: Timestamp) -> Result<Self, Self::Error> {
      timestamp.normalize();

      let chrono_utc: DateTime<Utc> = timestamp.try_into()?;

      Ok(chrono_utc.into())
    }
  }
}

#[cfg(feature = "totokens")]
mod totokens {
  use proc_macro2::TokenStream;
  use quote::{quote, ToTokens};

  use crate::Timestamp;

  impl ToTokens for Timestamp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let seconds = self.seconds;
      let nanos = self.nanos;

      tokens.extend(quote! {
        ::protocheck::types::Timestamp {
          seconds: #seconds,
          nanos: #nanos,
        }
      });
    }
  }
}
