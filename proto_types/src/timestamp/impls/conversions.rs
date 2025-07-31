use std::convert::{From, TryFrom};

use chrono::{DateTime, Utc};
use quote::{quote, ToTokens};

use crate::{Timestamp, TokenStream2};

impl From<DateTime<Utc>> for Timestamp {
  fn from(datetime: DateTime<Utc>) -> Self {
    let mut ts = Timestamp {
      seconds: datetime.timestamp(),
      nanos: datetime.timestamp_subsec_nanos() as i32,
    };
    ts.normalize();
    ts
  }
}

impl TryFrom<Timestamp> for DateTime<Utc> {
  type Error = String;

  fn try_from(mut timestamp: Timestamp) -> Result<Self, Self::Error> {
    timestamp.normalize();

    DateTime::<Utc>::from_timestamp(timestamp.seconds, timestamp.nanos as u32).ok_or_else(|| {
      format!(
        "Timestamp value (seconds: {}, nanos: {}) is out of DateTime<Utc> range or invalid.",
        timestamp.seconds, timestamp.nanos
      )
    })
  }
}

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
