use std::{
  cmp::Ordering,
  ops::{Add, Sub},
};

use chrono::{DateTime, Utc};

use crate::{Duration, Timestamp};

impl<'b> Sub<&'b Duration> for &Timestamp {
  type Output = Timestamp;

  fn sub(self, rhs: &'b Duration) -> Self::Output {
    let duration = rhs.normalized();

    let mut new = Timestamp {
      seconds: self.seconds - duration.seconds,
      nanos: self.nanos - duration.nanos,
    };

    new.normalize();

    new
  }
}

impl Sub<Duration> for Timestamp {
  type Output = Timestamp;
  fn sub(self, rhs: Duration) -> Self::Output {
    <&Timestamp as Sub<&Duration>>::sub(&self, &rhs)
  }
}

impl<'b> Sub<&'b Duration> for Timestamp {
  type Output = Timestamp;
  fn sub(self, rhs: &'b Duration) -> Self::Output {
    <&Timestamp as Sub<&Duration>>::sub(&self, rhs)
  }
}

impl<'a> Sub<Duration> for &'a Timestamp {
  type Output = Timestamp;
  fn sub(self, rhs: Duration) -> Self::Output {
    <&'a Timestamp as Sub<&Duration>>::sub(self, &rhs)
  }
}

impl<'b> Add<&'b Duration> for &Timestamp {
  type Output = Timestamp;

  fn add(self, rhs: &'b Duration) -> Self::Output {
    let duration = rhs.normalized();

    let mut new = Timestamp {
      seconds: self.seconds + duration.seconds,
      nanos: self.nanos + duration.nanos,
    };

    new.normalize();

    new
  }
}

impl<'b> Add<&'b Duration> for Timestamp {
  type Output = Timestamp;
  fn add(self, rhs: &'b Duration) -> Self::Output {
    <&Timestamp as Add<&Duration>>::add(&self, rhs)
  }
}

impl Add<Duration> for &Timestamp {
  type Output = Timestamp;
  fn add(self, rhs: Duration) -> Self::Output {
    <Self as Add<&Duration>>::add(self, &rhs)
  }
}

impl Add<Duration> for Timestamp {
  type Output = Timestamp;

  fn add(self, rhs: Duration) -> Self::Output {
    &self + &rhs
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
