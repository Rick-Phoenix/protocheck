use thiserror::Error;

use crate::common::Color;
use crate::{String, ToString};

/// Errors that can occur during the creation, conversion or validation of a [`Color`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ColorError {
  #[error("The value for red has to be between 0 and 1.")]
  InvalidRed,
  #[error("The value for green has to be between 0 and 1.")]
  InvalidGreen,
  #[error("The value for blue has to be between 0 and 1.")]
  InvalidBlue,
  #[error("The value for alpha has to be between 0 and 1.")]
  InvalidAlpha,
}

fn validate_color(red: f32, green: f32, blue: f32, alpha: Option<f32>) -> Result<(), ColorError> {
  let is_component_valid = |c: f32| (0.0..=1.0).contains(&c);

  if !is_component_valid(red) {
    Err(ColorError::InvalidRed)
  } else if !is_component_valid(green) {
    Err(ColorError::InvalidGreen)
  } else if !is_component_valid(blue) {
    Err(ColorError::InvalidBlue)
  } else if let Some(a) = alpha
    && !is_component_valid(a)
  {
    Err(ColorError::InvalidAlpha)
  } else {
    Ok(())
  }
}

impl Color {
  /// Creates a new [`Color`] instance. Returns a [`ColorError`] if one of the values is invalid.
  #[inline]
  pub fn new(red: f32, green: f32, blue: f32, alpha: Option<f32>) -> Result<Self, ColorError> {
    validate_color(red, green, blue, alpha)?;

    Ok(Self {
      red,
      green,
      blue,
      alpha: alpha.map(|value| crate::protobuf::FloatValue { value }),
    })
  }

  /// Validates the [`Color`] instance.
  #[inline]
  pub fn validate(&self) -> Result<(), ColorError> {
    validate_color(
      self.red,
      self.green,
      self.blue,
      Some(self.effective_alpha()),
    )
  }

  #[must_use]
  #[inline]
  /// Checks if the values are valid (i.e. they all range from 0 to 1.0).
  /// Redundant in case the constructor was used.
  pub fn is_valid(&self) -> bool {
    self.validate().is_ok()
  }

  #[must_use]
  #[inline]
  /// Returns the alpha or falls back to 1.0 as a default, as per the proto spec.
  pub fn effective_alpha(&self) -> f32 {
    self.alpha.as_ref().map_or(1.0, |fv| fv.value)
  }

  #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
  #[inline]
  /// Converts the value to rgba8, if it's valid.
  pub fn to_rgba8(&self) -> Result<(u8, u8, u8, u8), ColorError> {
    self.validate()?;
    // Safe castings after validation
    let r = (self.red * 255.0).round() as u8;
    let g = (self.green * 255.0).round() as u8;
    let b = (self.blue * 255.0).round() as u8;
    let a = (self.effective_alpha() * 255.0).round() as u8;
    Ok((r, g, b, a))
  }

  #[must_use]
  #[inline]
  /// Converts an rgba8 color to a [`Color`].
  pub fn from_rgba8(r: u8, g: u8, b: u8, a: Option<u8>) -> Self {
    Self {
      red: f32::from(r) / 255.0,
      green: f32::from(g) / 255.0,
      blue: f32::from(b) / 255.0,
      alpha: a.map(|value| crate::protobuf::FloatValue {
        value: f32::from(value) / 255.0,
      }),
    }
  }

  #[must_use]
  /// Returns an rgba string representation for this [`Color`].
  pub fn to_rgba_str(&self) -> String {
    self.to_string()
  }
}

impl core::fmt::Display for Color {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(
      f,
      "rgba({}, {}, {}, {:.1})",
      (self.red * 255.0).round(),
      (self.green * 255.0).round(),
      (self.blue * 255.0).round(),
      self.effective_alpha()
    )
  }
}

#[cfg(feature = "palette")]
mod palette {
  use palette::{Hsla, Oklch, Srgba, convert::IntoColor};

  use crate::Color;

  impl From<Color> for Srgba {
    #[inline]
    fn from(value: Color) -> Self {
      Self::new(value.red, value.green, value.blue, value.effective_alpha())
    }
  }

  impl From<Srgba> for Color {
    #[inline]
    fn from(value: Srgba) -> Self {
      Self {
        red: value.red,
        green: value.green,
        blue: value.blue,
        alpha: Some(crate::FloatValue { value: value.alpha }),
      }
    }
  }

  impl Color {
    #[must_use]
    #[inline]
    /// Convers this [`Color`] to [`palette::Hsla`]
    pub fn to_hsla(&self) -> Hsla {
      let srgba: Srgba = (*self).into();
      srgba.into_color()
    }

    #[must_use]
    #[inline]
    /// Convers this [`Color`] to [`palette::Oklch`]
    pub fn to_oklch(&self) -> Oklch {
      let srgba: Srgba = (*self).into();
      srgba.into_color()
    }
  }
}
