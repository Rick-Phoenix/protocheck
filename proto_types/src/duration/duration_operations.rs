use std::ops::{Add, Div, Mul, Sub};

use crate::{constants::NANOS_PER_SECOND, Duration};

impl Add for Duration {
  type Output = Duration;
  fn add(self, rhs: Self) -> Self::Output {
    self
      .checked_add(&rhs)
      .expect("Duration addition overflowed")
  }
}

impl Sub for Duration {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    self
      .checked_sub(&other)
      .expect("Duration subtraction overflowed")
  }
}

impl Mul<i64> for Duration {
  type Output = Self;

  fn mul(self, rhs: i64) -> Self {
    self
      .checked_mul(rhs)
      .expect("Duration multiplication by i64 overflowed")
  }
}

impl Mul<i32> for Duration {
  type Output = Self;

  fn mul(self, rhs: i32) -> Self {
    self
      .checked_mul(rhs as i64) // Simply cast to i64 and use the i64 implementation
      .expect("Duration multiplication by i32 overflowed")
  }
}

impl Div<i64> for Duration {
  type Output = Self;

  fn div(self, rhs: i64) -> Self {
    self
      .checked_div(rhs)
      .expect("Duration division by i64 overflowed or divided by zero")
  }
}

impl Div<i32> for Duration {
  type Output = Self;

  fn div(self, rhs: i32) -> Self {
    self
      .checked_div(rhs as i64)
      .expect("Duration division by i32 overflowed or divided by zero")
  }
}

impl Duration {
  /// Multiplies the Duration by an i64 scalar, returning `Some(Duration)` or `None` on overflow.
  pub fn checked_mul(&self, rhs: i64) -> Option<Self> {
    if rhs == 0 {
      return Some(Duration {
        seconds: 0,
        nanos: 0,
      });
    }

    let mut multiplied_nanos: i64 = (self.nanos as i64).checked_mul(rhs)?;

    let mut multiplied_seconds: i64 = self.seconds.checked_mul(rhs)?;

    let nanos_carry_to_seconds: i64 = multiplied_nanos / (NANOS_PER_SECOND as i64);
    multiplied_nanos %= NANOS_PER_SECOND as i64;

    multiplied_seconds = multiplied_seconds.checked_add(nanos_carry_to_seconds)?;

    let final_nanos = multiplied_nanos as i32;

    let mut result = Duration {
      seconds: multiplied_seconds,
      nanos: final_nanos,
    };

    let pre_normalize_seconds = result.seconds;
    result.normalize();

    if (result.seconds == i64::MAX && pre_normalize_seconds != i64::MAX)
      || (result.seconds == i64::MIN && pre_normalize_seconds != i64::MIN)
    {
      return None; // Overflow/underflow detected during or after normalization carry
    }

    Some(result)
  }

  /// Adds another Duration to this one, returning `Some(Duration)` or `None` on overflow.
  pub fn checked_add(&self, other: &Duration) -> Option<Self> {
    let sum_nanos = self.nanos.checked_add(other.nanos)?;

    let sum_seconds = self.seconds.checked_add(other.seconds)?;

    let mut result = Duration {
      seconds: sum_seconds,
      nanos: sum_nanos,
    };

    let pre_normalize_seconds = result.seconds;
    result.normalize();

    if (result.seconds == i64::MAX && pre_normalize_seconds != i64::MAX)
      || (result.seconds == i64::MIN && pre_normalize_seconds != i64::MIN)
    {
      return None; // Overflow/underflow detected during or after normalization carry
    }

    Some(result)
  }

  /// Subtracts another Duration from this one, returning `Some(Duration)` or `None` on overflow.
  pub fn checked_sub(&self, other: &Duration) -> Option<Self> {
    let diff_nanos = self.nanos.checked_sub(other.nanos)?;

    let diff_seconds = self.seconds.checked_sub(other.seconds)?;

    let mut result = Duration {
      seconds: diff_seconds,
      nanos: diff_nanos,
    };

    let pre_normalize_seconds = result.seconds;
    result.normalize();

    if (result.seconds == i64::MAX && pre_normalize_seconds != i64::MAX)
      || (result.seconds == i64::MIN && pre_normalize_seconds != i64::MIN)
    {
      return None; // Overflow/underflow detected during or after normalization carry
    }
    Some(result)
  }

  /// Divides the Duration by an i64 scalar, returning `Some(Duration)` or `None` on overflow.
  pub fn checked_div(&self, rhs: i64) -> Option<Self> {
    if rhs == 0 {
      return None;
    }

    if self.seconds == 0 && self.nanos == 0 {
      return Some(Duration {
        seconds: 0,
        nanos: 0,
      });
    }

    let divided_seconds = self.seconds.checked_div(rhs)?;
    let seconds_remainder = self.seconds.checked_rem(rhs)?;

    let nanos_from_remainder = seconds_remainder.checked_mul(NANOS_PER_SECOND as i64)?;

    // Combine original nanos with nanos from remainder, using i64 for arithmetic.
    let total_nanos_for_division = (self.nanos as i64).checked_add(nanos_from_remainder)?;

    let divided_nanos = total_nanos_for_division.checked_div(rhs)?;

    // `divided_nanos` should be within the i32 range after division, so cast is safe.
    let mut result = Duration {
      seconds: divided_seconds,
      nanos: divided_nanos as i32,
    };

    let pre_normalize_seconds = result.seconds;
    result.normalize();

    if (result.seconds == i64::MAX && pre_normalize_seconds != i64::MAX)
      || (result.seconds == i64::MIN && pre_normalize_seconds != i64::MIN)
    {
      return None; // Overflow/underflow detected during or after normalization carry
    }

    Some(result)
  }
}
