use core::{cmp::Ordering, fmt::Display};

use thiserror::Error;

use crate::common::Fraction;

impl Display for Fraction {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}/{}", self.numerator, self.denominator)
  }
}

/// Errors that can occur during the creation, conversion or validation of a [`Fraction`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum FractionError {
  #[error("Denominator cannot be zero")]
  ZeroDenominator,
  #[error("Fraction arithmetic operation resulted in an overflow")]
  Overflow,
  #[error("Fraction arithmetic operation resulted in an undefined state")]
  Undefined,
}

impl Fraction {
  /// Helper to calculate Greatest Common Divisor (GCD)
  #[must_use]
  #[inline]
  pub const fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
      let temp = b;
      b = a % b;
      a = temp;
    }
    a.abs()
  }

  /// Helper to calculate Least Common Multiple (LCM)
  #[inline]
  pub fn lcm(a: i64, b: i64) -> Result<i128, FractionError> {
    if a == 0 || b == 0 {
      return Err(FractionError::ZeroDenominator);
    }
    let common_divisor = i128::from(Self::gcd(a, b));
    let val_a = i128::from(a);
    let val_b = i128::from(b);

    let term1 = val_a
      .checked_div(common_divisor)
      .ok_or(FractionError::Overflow)?;
    term1
      .checked_mul(val_b)
      .ok_or(FractionError::Overflow)
  }

  /// Creates a new Fraction, ensuring the denominator is positive
  /// and the fraction is reduced to its simplest form.
  #[inline]
  pub const fn new(numerator: i64, denominator: i64) -> Result<Self, FractionError> {
    if denominator == 0 {
      return Err(FractionError::ZeroDenominator);
    }

    let (mut num, mut den) = (numerator, denominator);

    // Ensure denominator is positive, sign is carried by numerator
    if den < 0 {
      num = -num;
      den = -den;
    }

    let common_divisor = Self::gcd(num, den);
    Ok(Self {
      numerator: num / common_divisor,
      denominator: den / common_divisor,
    })
  }

  /// Reduces the fraction to its simplest form by dividing
  /// numerator and denominator by their greatest common divisor.
  #[inline]
  pub const fn reduce(&mut self) {
    if self.denominator == 0 {
      return;
    }

    // Ensure denominator is positive, sign is carried by numerator
    if self.denominator < 0 {
      self.numerator = -self.numerator;
      self.denominator = -self.denominator;
    }

    let common_divisor = Self::gcd(self.numerator, self.denominator);
    self.numerator /= common_divisor;
    self.denominator /= common_divisor;
  }

  /// Returns a new, reduced Fraction.
  #[must_use]
  #[inline]
  pub const fn reduced(mut self) -> Self {
    self.reduce();
    self
  }

  /// Checked addition for [`Fraction`]s.
  #[inline]
  pub fn checked_add(self, other: Self) -> Result<Self, FractionError> {
    let common_denominator_i128 = Self::lcm(self.denominator, other.denominator)?;

    let factor_self = common_denominator_i128
      .checked_div(i128::from(self.denominator))
      .ok_or(FractionError::Overflow)?;

    let factor_other = common_denominator_i128
      .checked_div(i128::from(other.denominator))
      .ok_or(FractionError::Overflow)?;

    let new_numerator_left = i128::from(self.numerator)
      .checked_mul(factor_self)
      .ok_or(FractionError::Overflow)?;

    let new_numerator_right = i128::from(other.numerator)
      .checked_mul(factor_other)
      .ok_or(FractionError::Overflow)?;

    let new_numerator = new_numerator_left
      .checked_add(new_numerator_right)
      .ok_or(FractionError::Overflow)?;

    let num_i64 = i64::try_from(new_numerator).map_err(|_| FractionError::Overflow)?;
    let den_i64 = i64::try_from(common_denominator_i128).map_err(|_| FractionError::Overflow)?;

    Self::new(num_i64, den_i64)
  }

  /// Checked subtraction for [`Fraction`]s.
  #[inline]
  pub fn checked_sub(self, other: Self) -> Result<Self, FractionError> {
    let common_denominator_i128 = Self::lcm(self.denominator, other.denominator)?;

    let factor_self = common_denominator_i128
      .checked_div(i128::from(self.denominator))
      .ok_or(FractionError::Overflow)?;

    let factor_other = common_denominator_i128
      .checked_div(i128::from(other.denominator))
      .ok_or(FractionError::Overflow)?;

    let new_numerator_left = i128::from(self.numerator)
      .checked_mul(factor_self)
      .ok_or(FractionError::Overflow)?;

    let new_numerator_right = i128::from(other.numerator)
      .checked_mul(factor_other)
      .ok_or(FractionError::Overflow)?;

    let new_numerator = new_numerator_left
      .checked_sub(new_numerator_right)
      .ok_or(FractionError::Overflow)?;

    let num_i64 = i64::try_from(new_numerator).map_err(|_| FractionError::Overflow)?;
    let den_i64 = i64::try_from(common_denominator_i128).map_err(|_| FractionError::Overflow)?;

    Self::new(num_i64, den_i64)
  }

  /// Checked multiplication for [`Fraction`]s.
  #[inline]
  pub fn checked_mul(self, other: Self) -> Result<Self, FractionError> {
    let new_numerator = i128::from(self.numerator)
      .checked_mul(i128::from(other.numerator))
      .ok_or(FractionError::Overflow)?;

    let new_denominator = i128::from(self.denominator)
      .checked_mul(i128::from(other.denominator))
      .ok_or(FractionError::Overflow)?;

    let num_i64 = i64::try_from(new_numerator).map_err(|_| FractionError::Overflow)?;
    let den_i64 = i64::try_from(new_denominator).map_err(|_| FractionError::Overflow)?;

    Self::new(num_i64, den_i64)
  }

  /// Checked division for [`Fraction`]s.
  #[inline]
  pub fn checked_div(self, other: Self) -> Result<Self, FractionError> {
    if other.numerator == 0 {
      return Err(FractionError::Undefined);
    }

    let new_numerator = i128::from(self.numerator)
      .checked_mul(i128::from(other.denominator))
      .ok_or(FractionError::Overflow)?;

    let new_denominator = i128::from(self.denominator)
      .checked_mul(i128::from(other.numerator))
      .ok_or(FractionError::Overflow)?;

    let num_i64 = i64::try_from(new_numerator).map_err(|_| FractionError::Overflow)?;
    let den_i64 = i64::try_from(new_denominator).map_err(|_| FractionError::Overflow)?;

    Self::new(num_i64, den_i64)
  }

  /// Converts the fraction to an `f64`.
  ///
  /// # Panics
  /// Panics if the denominator is zero. This should not happen for [`Fraction`]
  /// instances created via [`Fraction::new()`] or other checked arithmetic,
  /// but can occur if a [`Fraction`] is constructed directly in an invalid state.
  ///
  /// For a fallible conversion that returns a `Result`, use `TryFrom<Fraction> for f64`.
  #[must_use]
  #[inline]
  pub fn to_f64_unchecked(self) -> f64 {
    self.try_into().unwrap()
  }
}

impl PartialOrd for Fraction {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.denominator <= 0 || other.denominator <= 0 {
      return None;
    }
    let self_val = i128::from(self.numerator) * i128::from(other.denominator);
    let other_val = i128::from(other.numerator) * i128::from(self.denominator);

    Some(self_val.cmp(&other_val))
  }
}

impl TryFrom<Fraction> for f64 {
  type Error = FractionError;
  #[inline]
  fn try_from(fraction: Fraction) -> Result<Self, Self::Error> {
    if fraction.denominator == 0 {
      return Err(FractionError::ZeroDenominator);
    }

    let num_f64 = fraction.numerator as Self;
    let den_f64 = fraction.denominator as Self;

    Ok(num_f64 / den_f64)
  }
}
