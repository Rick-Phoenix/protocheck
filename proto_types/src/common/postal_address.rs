use crate::common::PostalAddress;

impl PostalAddress {
  /// Checks if this [`PostalAddress`]'s `region_code` is empty. If it is, it means that the instance is invalid.
  pub fn has_region_code(&self) -> bool {
    !self.region_code.is_empty()
  }

  /// Checks if the `region_code` of this address matches the given `code`.
  /// The `code` should be a CLDR region code (ISO 3166-1 alpha-2, e.g., "US", "CH").
  pub fn matches_region_code(&self, code: &str) -> bool {
    self.region_code == code
  }

  /// Checks if the `language_code` of this address matches the given BCP-47 `code`.
  /// The `code` should be a BCP-44 language tag (e.g., "en-US", "ja").
  pub fn has_language_code(&self, code: &str) -> bool {
    self.language_code == code
  }

  /// Checks if the `postal_code` of this address matches the given `code`.
  pub fn has_postal_code(&self, code: &str) -> bool {
    self.postal_code == code
  }

  /// Checks if the `sorting_code` of this address matches the given `code`.
  pub fn has_sorting_code(&self, code: &str) -> bool {
    self.sorting_code == code
  }

  /// Checks if the `administrative_area` of this address matches the given `name`.
  pub fn has_administrative_area(&self, name: &str) -> bool {
    self.administrative_area == name
  }

  /// Checks if the `locality` (city/town) of this address matches the given `name`.
  pub fn has_locality(&self, name: &str) -> bool {
    self.locality == name
  }

  /// Checks if the `sublocality` of this address matches the given `name`.
  pub fn has_sublocality(&self, name: &str) -> bool {
    self.sublocality == name
  }
}
