use std::{
  cmp::Ordering,
  ops::{Add, Sub},
};

use crate::{Duration, Timestamp};

impl<'b> Sub<&'b Duration> for &Timestamp {
  type Output = Timestamp;

  fn sub(self, rhs: &'b Duration) -> Self::Output {
    let duration = rhs.normalized();

    let mut new = Timestamp {
      seconds: self.seconds.saturating_sub(duration.seconds),
      nanos: self.nanos - duration.nanos,
    };

    new.normalize();

    new
  }
}

impl Sub<Duration> for Timestamp {
  type Output = Self;
  fn sub(self, rhs: Duration) -> Self::Output {
    <&Self as Sub<&Duration>>::sub(&self, &rhs)
  }
}

impl<'b> Sub<&'b Duration> for Timestamp {
  type Output = Self;
  fn sub(self, rhs: &'b Duration) -> Self::Output {
    <&Self as Sub<&Duration>>::sub(&self, rhs)
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
      seconds: self.seconds.saturating_add(duration.seconds),
      nanos: self.nanos + duration.nanos,
    };

    new.normalize();

    new
  }
}

impl<'b> Add<&'b Duration> for Timestamp {
  type Output = Self;
  fn add(self, rhs: &'b Duration) -> Self::Output {
    <&Self as Add<&Duration>>::add(&self, rhs)
  }
}

impl Add<Duration> for &Timestamp {
  type Output = Timestamp;
  fn add(self, rhs: Duration) -> Self::Output {
    <Self as Add<&Duration>>::add(self, &rhs)
  }
}

impl Add<Duration> for Timestamp {
  type Output = Self;

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
    (self.seconds, self.nanos).cmp(&(other.seconds, other.nanos))
  }
}
