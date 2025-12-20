//! Implementations for the google.type.Money message.
//!
//!
//! DISCLAIMER: all of the methods implemented for Money are just implemented for convenience, and they are provided as is, without warranties of any kind. By using this module, the user is relieving the authors of this library from any responsibility for any damage that may be caused by its usage.

use std::cmp::Ordering;
use std::fmt::Write;

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
    let units_carry = i64::from(nanos / (NANO_FACTOR));
    units = units
      .checked_add(units_carry)
      .ok_or(MoneyError::Overflow)?;
    nanos %= NANO_FACTOR;
  }

  if units > 0 && nanos < 0 {
    units = units
      .checked_sub(1)
      .ok_or(MoneyError::Underflow)?;
    nanos = nanos
      .checked_add(NANO_FACTOR)
      .ok_or(MoneyError::Overflow)?;
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
    let self_total_nanos = i128::from(self.units) * 1_000_000_000i128 + i128::from(self.nanos);
    let other_total_nanos = i128::from(other.units) * 1_000_000_000i128 + i128::from(other.nanos);

    self_total_nanos.partial_cmp(&other_total_nanos)
  }
}

impl Money {
  /// Normalizes the [`Money`] amount and returns a string containing the currency symbol and the monetary amount rounded by the specified decimal places.
  #[must_use]
  pub fn to_formatted_string(&self, symbol: &str, decimal_places: u32) -> String {
    let decimal_places = u32::min(9, decimal_places);

    let mut current_units: i128 = i128::from(self.units);
    let mut current_nanos: i128 = i128::from(self.nanos);

    let ten_pow_9 = i128::from(NANO_FACTOR);
    if current_nanos >= ten_pow_9 || current_nanos <= -ten_pow_9 {
      current_units += current_nanos / ten_pow_9;
      current_nanos %= ten_pow_9;
    }

    if current_units > 0 && current_nanos < 0 {
      current_units -= 1;
      current_nanos += ten_pow_9;
    } else if current_units < 0 && current_nanos > 0 {
      current_units += 1;
      current_nanos -= ten_pow_9;
    }

    let mut rounded_nanos = 0;
    let mut units_carry = 0;

    if decimal_places > 0 {
      let power_of_10_for_display = 10_i128.pow(decimal_places);
      let rounding_power = 10_i128.pow(9 - decimal_places);

      let abs_nanos = current_nanos.abs();

      let remainder_for_rounding = abs_nanos % rounding_power;
      rounded_nanos = abs_nanos / rounding_power;

      if remainder_for_rounding >= rounding_power / 2 {
        rounded_nanos += 1;
      }

      // Handle carry-over from nanos rounding to units
      if rounded_nanos >= power_of_10_for_display {
        units_carry = 1;
        rounded_nanos = 0;
      }
    }

    let is_negative = current_units < 0 || (current_units == 0 && current_nanos < 0);

    let final_units_abs = current_units.abs() + units_carry;

    let mut formatted_string = String::new();

    if is_negative {
      formatted_string.push('-');
    }
    formatted_string.push_str(symbol);
    formatted_string.push_str(&final_units_abs.to_string());

    if decimal_places > 0 {
      formatted_string.push('.');
      // Format rounded_nanos to the specified number of decimal places, zero-padded
      let _ = write!(
        formatted_string,
        "{:0width$}",
        rounded_nanos,
        width = decimal_places as usize
      );
    }

    formatted_string
  }

  /// Normalizes units and nanos. Fails in case of overflow.
  pub fn normalize(mut self) -> Result<Self, MoneyError> {
    let (normalized_units, normalized_nanos) =
      normalize_money_fields_checked(self.units, self.nanos)?;
    self.units = normalized_units;
    self.nanos = normalized_nanos;

    Ok(self)
  }

  /// Creates a new instance, if the normalization does not return errors like Overflow or Underflow.
  pub fn new(currency_code: String, units: i64, nanos: i32) -> Result<Self, MoneyError> {
    let (normalized_units, normalized_nanos) = normalize_money_fields_checked(units, nanos)?;
    Ok(Self {
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
  pub fn to_rounded_imprecise_f64(&self, decimal_places: u32) -> Result<f64, MoneyError> {
    if decimal_places > i32::MAX as u32 {
      return Err(MoneyError::Overflow);
    }

    let full_amount = self.as_imprecise_f64();

    let factor_exponent: i32 = decimal_places
      .try_into()
      .map_err(|_| MoneyError::Overflow)?;
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
  #[must_use]
  pub fn as_imprecise_f64(&self) -> f64 {
    self.units as f64 + (f64::from(self.nanos) / 1_000_000_000.0)
  }

  /// Creates a new `Money` instance with the given currency code and decimal amount.
  ///
  /// This is a convenience constructor that handles splitting a decimal value
  /// into units and nanos.
  ///
  /// WARNING: The usage of `f64` introduces floating-point precision issues. Do not use it for critical financial calculations.
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

    // SAFETY: We already truncateda, and this cast is safe because we checked the range
    #[allow(clippy::cast_possible_truncation)]
    let units = truncated_amount as i64;

    let raw_nanos_f64 = amount.fract().abs() * f64::from(NANO_FACTOR);
    // SAFETY: The range is guaranteed to be 0..1,000,000,000 by logic.
    #[allow(clippy::cast_possible_truncation)]
    let nanos: i32 = raw_nanos_f64.round() as i32;

    let final_nanos = if units < 0 && nanos > 0 {
      -nanos
    } else if units == 0 && amount < 0.0 && nanos > 0 {
      // For -0.5, ensure nanos is -500M
      -nanos
    } else {
      nanos
    };

    Self::new(currency_code, units, final_nanos)
  }

  /// Attempts to add another [`Money`] amount to this one, returning a new [`Money`] instance.
  /// Returns an error if currencies mismatch or if addition causes an overflow/underflow.
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

    Self::new(self.currency_code.clone(), sum_units, sum_nanos)
  }

  /// Attempts to add another [`Money`] amount to this one in place.
  /// Returns an error if currencies mismatch or if addition causes an overflow/underflow.
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

    Self::new(self.currency_code.clone(), diff_units, diff_nanos)
  }

  /// Attempts to subtract another [`Money`] amount from this one in place.
  /// Returns an error if currencies mismatch or if subtraction causes an overflow/underflow.
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
  pub fn try_mul_i64(&self, rhs: i64) -> Result<Self, MoneyError> {
    let mul_units = self
      .units
      .checked_mul(rhs)
      .ok_or(MoneyError::Overflow)?;
    let mul_nanos_i64 = i64::from(self.nanos)
      .checked_mul(rhs)
      .ok_or(MoneyError::Overflow)?;

    let final_nanos_for_new: i32 = mul_nanos_i64
      .try_into()
      .map_err(|_| MoneyError::Overflow)?;

    Self::new(self.currency_code.clone(), mul_units, final_nanos_for_new)
  }

  /// Attempts to multiply this [`Money`] amount by a float scalar, returning a new [`Money`] instance.
  /// Returns an error if the result is non-finite or causes an internal conversion error.
  /// WARNING: The usage of `f64` introduces floating-point precision issues. Do not use it for critical financial calculations.
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
    Self::from_imprecise_f64(self.currency_code.clone(), result_decimal)
  }

  /// Attempts to divide this [`Money`] amount by an integer scalar, returning a new [`Money`] instance.
  /// Returns an error if the divisor is zero, or if division causes an overflow/underflow.
  pub fn try_div_i64(&self, rhs: i64) -> Result<Self, MoneyError> {
    if rhs == 0 {
      return Err(MoneyError::InvalidAmount); // Division by zero
    }

    let total_nanos_i128 =
      i128::from(self.units) * i128::from(NANO_FACTOR) + i128::from(self.nanos);

    let result_total_nanos = total_nanos_i128
      .checked_div(i128::from(rhs))
      .ok_or(MoneyError::Overflow)?;

    // Safely convert the `new_units` from i128 to i64
    let new_units_i128 = result_total_nanos / i128::from(NANO_FACTOR);
    let new_units = new_units_i128
      .try_into()
      .map_err(|_| MoneyError::Overflow)?;

    // This cast is safe because (result % NANO_FACTOR) is always < NANO_FACTOR,
    // and NANO_FACTOR itself fits in i32.
    #[allow(clippy::cast_possible_truncation)]
    let new_nanos = (result_total_nanos % i128::from(NANO_FACTOR)) as i32;

    Self::new(self.currency_code.clone(), new_units, new_nanos)
  }

  /// Attempts to divide this [`Money`] amount by a float scalar, returning a new [`Money`] instance.
  /// Returns an error if the divisor is zero, non-finite, or if division causes an internal conversion error.
  /// WARNING: The usage of `f64` introduces floating-point precision issues. Do not use it for critical financial calculations.
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

    Self::from_imprecise_f64(self.currency_code.clone(), result_decimal)
  }

  /// Attempts to negate this [`Money`] amount, returning a new [`Money`] instance.
  /// Returns an error if negation causes an overflow/underflow.
  pub fn try_neg(&self) -> Result<Self, MoneyError> {
    let neg_units = self
      .units
      .checked_neg()
      .ok_or(MoneyError::Overflow)?;
    let neg_nanos = self
      .nanos
      .checked_neg()
      .ok_or(MoneyError::Overflow)?;

    Self::new(self.currency_code.clone(), neg_units, neg_nanos)
  }

  /// Checks if the money's currency code matches the given `code`.
  /// The `code` should be a three-letter ISO 4217 currency code (e.g., "USD", "EUR").
  #[must_use]
  pub fn is_currency(&self, code: &str) -> bool {
    self.currency_code == code
  }

  /// Checks if the money's currency is United States Dollar (USD).
  #[must_use]
  pub fn is_usd(&self) -> bool {
    self.is_currency("USD")
  }

  /// Checks if the money's currency is Euro (EUR).
  #[must_use]
  pub fn is_eur(&self) -> bool {
    self.is_currency("EUR")
  }

  /// Checks if the money's currency is British Pound Sterling (GBP).
  #[must_use]
  pub fn is_gbp(&self) -> bool {
    self.is_currency("GBP")
  }

  /// Checks if the money's currency is Japanese Yen (JPY).
  #[must_use]
  pub fn is_jpy(&self) -> bool {
    self.is_currency("JPY")
  }

  /// Checks if the money's currency is Canadian Dollar (CAD).
  #[must_use]
  pub fn is_cad(&self) -> bool {
    self.is_currency("CAD")
  }

  /// Checks if the money's currency is Australian Dollar (AUD).
  #[must_use]
  pub fn is_aud(&self) -> bool {
    self.is_currency("AUD")
  }

  /// Checks if the money amount is strictly positive (greater than zero).
  #[must_use]
  pub const fn is_positive(&self) -> bool {
    self.units > 0 || (self.units == 0 && self.nanos > 0)
  }

  /// Checks if the money amount is strictly negative (less than zero).
  #[must_use]
  pub const fn is_negative(&self) -> bool {
    self.units < 0 || (self.units == 0 && self.nanos < 0)
  }

  /// Checks if the money amount is exactly zero.
  #[must_use]
  pub const fn is_zero(&self) -> bool {
    self.units == 0 && self.nanos == 0
  }
}
