//! DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.

use std::cmp::Ordering;

use thiserror::Error;

use crate::common::Money;

const NANO_FACTOR: i32 = 1_000_000_000;

/// Errors that can occur during the creation, conversion or validation of [`Money`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum MoneyError {
  #[error("Currency mismatch: Expected '{expected}', found '{found}'")]
  CurrencyMismatch { expected: String, found: String },
  #[error("Money arithmetic overflow")]
  Overflow,
  #[error("Money arithmetic underflow")]
  Underflow,
  #[error("Money amount resulted in an invalid state")]
  InvalidAmount,
  #[error("Floating point operation resulted in a non-finite number (NaN or Infinity)")]
  NonFiniteResult,
}

fn normalize_money_fields_checked(
  mut units: i64,
  mut nanos: i32,
) -> Result<(i64, i32), MoneyError> {
  if nanos.abs() >= NANO_FACTOR {
    let units_carry = (nanos / (NANO_FACTOR)) as i64;
    units = units.checked_add(units_carry).ok_or(MoneyError::Overflow)?;
    nanos %= NANO_FACTOR;
  }

  if units > 0 && nanos < 0 {
    units = units.checked_sub(1).ok_or(MoneyError::Underflow)?;
    nanos = nanos.checked_add(NANO_FACTOR).ok_or(MoneyError::Overflow)?;
  } else if units < 0 && nanos > 0 {
    units = units.checked_add(1).ok_or(MoneyError::Overflow)?;
    nanos = nanos
      .checked_sub(NANO_FACTOR)
      .ok_or(MoneyError::Underflow)?;
  }

  Ok((units, nanos))
}

impl PartialOrd for Money {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.currency_code != other.currency_code {
      return None; // Indicate incomparability
    }

    // To compare accurately, convert the amount to a single i128 value in nanos.
    let self_total_nanos = self.units as i128 * 1_000_000_000i128 + self.nanos as i128;
    let other_total_nanos = other.units as i128 * 1_000_000_000i128 + other.nanos as i128;

    self_total_nanos.partial_cmp(&other_total_nanos)
  }
}

impl Money {
  /// Creates a new instance, if the normalization does not return errors like Overflow or Underflow.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn new(currency_code: String, units: i64, nanos: i32) -> Result<Self, MoneyError> {
    let (normalized_units, normalized_nanos) = normalize_money_fields_checked(units, nanos)?;
    Ok(Money {
      currency_code,
      units: normalized_units,
      nanos: normalized_nanos,
    })
  }

  /// Converts the [`Money`] amount into a decimal (f64) representation,
  /// rounded to the specified number of decimal places.
  ///
  /// `decimal_places` determines the precision of the rounding. For example:
  /// - `0` rounds to the nearest whole unit.
  /// - `2` rounds to two decimal places (e.g., for cents).
  ///
  /// WARNING: The usage of `f64` introduces floating-point precision issues. Do not use it for critical financial calculations.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn to_rounded_imprecise_f64(&self, decimal_places: u32) -> Result<f64, MoneyError> {
    if decimal_places > i32::MAX as u32 {
      return Err(MoneyError::Overflow);
    }

    let full_amount = self.as_imprecise_f64();

    let factor_exponent = decimal_places as i32;
    let factor = 10.0f64.powi(factor_exponent);

    if !factor.is_finite() {
      return Err(MoneyError::NonFiniteResult);
    }

    let result = (full_amount * factor).round() / factor;

    if !result.is_finite() {
      return Err(MoneyError::NonFiniteResult);
    }

    Ok(result)
  }

  /// Converts the `Money` amount into a decimal (f64) representation.
  ///
  /// WARNING: The usage of `f64` introduces floating-point precision issues. Do not use it for critical financial calculations.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn as_imprecise_f64(&self) -> f64 {
    self.units as f64 + (self.nanos as f64 / 1_000_000_000.0)
  }

  /// Creates a new `Money` instance with the given currency code and decimal amount.
  ///
  /// This is a convenience constructor that handles splitting a decimal value
  /// into units and nanos.
  ///
  /// WARNING: The usage of `f64` introduces floating-point precision issues. Do not use it for critical financial calculations.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn from_imprecise_f64(currency_code: String, amount: f64) -> Result<Self, MoneyError> {
    if !amount.is_finite() {
      return Err(MoneyError::NonFiniteResult);
    }

    let truncated_amount = amount.trunc();

    if truncated_amount > i64::MAX as f64 {
      return Err(MoneyError::Overflow);
    } else if truncated_amount < i64::MIN as f64 {
      return Err(MoneyError::Underflow);
    }

    let units = truncated_amount as i64; // Now this cast is safe because we checked the range

    let raw_nanos_f64 = amount.fract().abs() * NANO_FACTOR as f64;
    let nanos: i32 = raw_nanos_f64.round() as i32;

    let final_nanos = if units < 0 && nanos > 0 {
      -nanos
    } else if units == 0 && amount < 0.0 && nanos > 0 {
      // For -0.5, ensure nanos is -500M
      -nanos
    } else {
      nanos
    };

    Money::new(currency_code, units, final_nanos)
  }

  /// Attempts to add another [`Money`] amount to this one, returning a new [`Money`] instance.
  /// Returns an error if currencies mismatch or if addition causes an overflow/underflow.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn try_add(&self, other: &Self) -> Result<Self, MoneyError> {
    if self.currency_code != other.currency_code {
      return Err(MoneyError::CurrencyMismatch {
        expected: self.currency_code.clone(),
        found: other.currency_code.clone(),
      });
    }

    let sum_units = self
      .units
      .checked_add(other.units)
      .ok_or(MoneyError::Overflow)?;
    let sum_nanos = self
      .nanos
      .checked_add(other.nanos)
      .ok_or(MoneyError::Overflow)?;

    Money::new(self.currency_code.clone(), sum_units, sum_nanos)
  }

  /// Attempts to add another [`Money`] amount to this one in place.
  /// Returns an error if currencies mismatch or if addition causes an overflow/underflow.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn try_add_assign(&mut self, other: &Self) -> Result<(), MoneyError> {
    if self.currency_code != other.currency_code {
      return Err(MoneyError::CurrencyMismatch {
        expected: self.currency_code.clone(),
        found: other.currency_code.clone(),
      });
    }

    self.units = self
      .units
      .checked_add(other.units)
      .ok_or(MoneyError::Overflow)?;
    self.nanos = self
      .nanos
      .checked_add(other.nanos)
      .ok_or(MoneyError::Overflow)?;

    let (final_units, final_nanos) = normalize_money_fields_checked(self.units, self.nanos)?;
    self.units = final_units;
    self.nanos = final_nanos;
    Ok(())
  }

  /// Attempts to subtract another [`Money`] amount from this one, returning a new [`Money`] instance.
  /// Returns an error if currencies mismatch or if subtraction causes an overflow/underflow.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn try_sub(&self, other: &Self) -> Result<Self, MoneyError> {
    if self.currency_code != other.currency_code {
      return Err(MoneyError::CurrencyMismatch {
        expected: self.currency_code.clone(),
        found: other.currency_code.clone(),
      });
    }

    let diff_units = self
      .units
      .checked_sub(other.units)
      .ok_or(MoneyError::Underflow)?;
    let diff_nanos = self
      .nanos
      .checked_sub(other.nanos)
      .ok_or(MoneyError::Underflow)?;

    Money::new(self.currency_code.clone(), diff_units, diff_nanos)
  }

  /// Attempts to subtract another [`Money`] amount from this one in place.
  /// Returns an error if currencies mismatch or if subtraction causes an overflow/underflow.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn try_sub_assign(&mut self, other: &Self) -> Result<(), MoneyError> {
    if self.currency_code != other.currency_code {
      return Err(MoneyError::CurrencyMismatch {
        expected: self.currency_code.clone(),
        found: other.currency_code.clone(),
      });
    }

    self.units = self
      .units
      .checked_sub(other.units)
      .ok_or(MoneyError::Underflow)?;
    self.nanos = self
      .nanos
      .checked_sub(other.nanos)
      .ok_or(MoneyError::Underflow)?;

    let (final_units, final_nanos) = normalize_money_fields_checked(self.units, self.nanos)?;
    self.units = final_units;
    self.nanos = final_nanos;
    Ok(())
  }

  /// Attempts to multiply this [`Money`] amount by an integer scalar, returning a new [`Money`] instance.
  /// Returns an error if multiplication causes an overflow/underflow.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn try_mul_i64(&self, rhs: i64) -> Result<Self, MoneyError> {
    let mul_units = self.units.checked_mul(rhs).ok_or(MoneyError::Overflow)?;
    let mul_nanos_i64 = (self.nanos as i64)
      .checked_mul(rhs)
      .ok_or(MoneyError::Overflow)?;

    let final_nanos_for_new: i32 = mul_nanos_i64.try_into().map_err(|_| MoneyError::Overflow)?;

    Money::new(self.currency_code.clone(), mul_units, final_nanos_for_new)
  }

  /// Attempts to multiply this [`Money`] amount by a float scalar, returning a new [`Money`] instance.
  /// Returns an error if the result is non-finite or causes an internal conversion error.
  /// WARNING: The usage of `f64` introduces floating-point precision issues. Do not use it for critical financial calculations.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn try_mul_f64(&self, rhs: f64) -> Result<Self, MoneyError> {
    if !rhs.is_finite() {
      return Err(MoneyError::NonFiniteResult);
    }

    let decimal_amount = self.as_imprecise_f64();
    let result_decimal = decimal_amount * rhs;

    if !result_decimal.is_finite() {
      return Err(MoneyError::NonFiniteResult);
    }

    // Pass the result to from_decimal_f64, which will normalize and validate.
    Money::from_imprecise_f64(self.currency_code.clone(), result_decimal)
  }

  /// Attempts to divide this [`Money`] amount by an integer scalar, returning a new [`Money`] instance.
  /// Returns an error if the divisor is zero, or if division causes an overflow/underflow.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn try_div_i64(&self, rhs: i64) -> Result<Self, MoneyError> {
    if rhs == 0 {
      return Err(MoneyError::InvalidAmount); // Division by zero
    }

    let total_nanos_i128 = self.units as i128 * NANO_FACTOR as i128 + self.nanos as i128;

    let result_total_nanos = total_nanos_i128
      .checked_div(rhs as i128)
      .ok_or(MoneyError::Overflow)?;

    // Safely convert the `new_units` from i128 to i64
    let new_units_i128 = result_total_nanos / NANO_FACTOR as i128;
    let new_units = new_units_i128
      .try_into()
      .map_err(|_| MoneyError::Overflow)?;

    // This cast is safe because (result % NANO_FACTOR) is always < NANO_FACTOR,
    // and NANO_FACTOR itself fits in i32.
    let new_nanos = (result_total_nanos % NANO_FACTOR as i128) as i32;

    Money::new(self.currency_code.clone(), new_units, new_nanos)
  }

  /// Attempts to divide this [`Money`] amount by a float scalar, returning a new [`Money`] instance.
  /// Returns an error if the divisor is zero, non-finite, or if division causes an internal conversion error.
  /// WARNING: The usage of `f64` introduces floating-point precision issues. Do not use it for critical financial calculations.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn try_div_f64(&self, rhs: f64) -> Result<Self, MoneyError> {
    if rhs == 0.0 {
      return Err(MoneyError::InvalidAmount);
    }
    if !rhs.is_finite() {
      return Err(MoneyError::NonFiniteResult);
    }

    let decimal_amount = self.as_imprecise_f64();
    let result_decimal = decimal_amount / rhs;

    if !result_decimal.is_finite() {
      return Err(MoneyError::NonFiniteResult);
    }

    Money::from_imprecise_f64(self.currency_code.clone(), result_decimal)
  }

  /// Attempts to negate this [`Money`] amount, returning a new [`Money`] instance.
  /// Returns an error if negation causes an overflow/underflow.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn try_neg(&self) -> Result<Self, MoneyError> {
    let neg_units = self.units.checked_neg().ok_or(MoneyError::Overflow)?;
    let neg_nanos = self.nanos.checked_neg().ok_or(MoneyError::Overflow)?;

    Money::new(self.currency_code.clone(), neg_units, neg_nanos)
  }

  /// Checks if the money's currency code matches the given `code`.
  /// The `code` should be a three-letter ISO 4217 currency code (e.g., "USD", "EUR").
  pub fn is_currency(&self, code: &str) -> bool {
    self.currency_code == code
  }

  /// Checks if the money's currency is United States Dollar (USD).
  pub fn is_usd(&self) -> bool {
    self.is_currency("USD")
  }

  /// Checks if the money's currency is Euro (EUR).
  pub fn is_eur(&self) -> bool {
    self.is_currency("EUR")
  }

  /// Checks if the money's currency is British Pound Sterling (GBP).
  pub fn is_gbp(&self) -> bool {
    self.is_currency("GBP")
  }

  /// Checks if the money's currency is Japanese Yen (JPY).
  pub fn is_jpy(&self) -> bool {
    self.is_currency("JPY")
  }

  /// Checks if the money's currency is Canadian Dollar (CAD).
  pub fn is_cad(&self) -> bool {
    self.is_currency("CAD")
  }

  /// Checks if the money's currency is Australian Dollar (AUD).
  pub fn is_aud(&self) -> bool {
    self.is_currency("AUD")
  }

  /// Checks if the money amount is strictly positive (greater than zero).
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn is_positive(&self) -> bool {
    self.units > 0 || (self.units == 0 && self.nanos > 0)
  }

  /// Checks if the money amount is strictly negative (less than zero).
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn is_negative(&self) -> bool {
    self.units < 0 || (self.units == 0 && self.nanos < 0)
  }

  /// Checks if the money amount is exactly zero.
  /// DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.
  pub fn is_zero(&self) -> bool {
    self.units == 0 && self.nanos == 0
  }
}
