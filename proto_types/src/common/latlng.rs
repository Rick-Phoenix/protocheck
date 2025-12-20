use thiserror::Error;

use crate::LatLng;

/// Errors that can occur during the creation or validation of a [`LatLng`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum LatLngError {
  #[error("Latitude out of valid range (-90.0, +90.0)")]
  InvalidLatitude,
  #[error("Longitude out of valid range (-180.0, +180.0)")]
  InvalidLongitude,
}

fn validate_latlng(latitude: f64, longitude: f64) -> Result<(), LatLngError> {
  if !((-90.0..=90.0).contains(&latitude)) {
    Err(LatLngError::InvalidLatitude)
  } else if !((-180.0..=180.0).contains(&longitude)) {
    Err(LatLngError::InvalidLongitude)
  } else {
    Ok(())
  }
}

impl LatLng {
  /// Creates a new instance. It fails if the latitude or longitude are not within the allowed ranges.
  pub fn new(latitude: f64, longitude: f64) -> Result<Self, LatLngError> {
    validate_latlng(latitude, longitude)?;

    Ok(Self {
      latitude,
      longitude,
    })
  }

  /// Validates the [`LatLng`] instance by checking if the values are within the allowed range.
  pub fn validate(&self) -> Result<(), LatLngError> {
    validate_latlng(self.latitude, self.longitude)
  }

  /// Checks if the [`LatLng`] instance contains valid values.
  #[must_use]
  pub fn is_valid(&self) -> bool {
    self.validate().is_ok()
  }
}

impl std::fmt::Display for LatLng {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:.6},{:.6}", self.latitude, self.longitude)
  }
}
