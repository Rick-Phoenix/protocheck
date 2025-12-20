use crate::common::LocalizedText;

impl LocalizedText {
  /// Checks if the language code matches the given input.
  #[must_use]
  pub fn has_code(&self, code: &str) -> bool {
    self.language_code == code
  }

  /// Checks if the language code is for English.
  /// This method checks for the primary 'en' subtag.
  #[must_use]
  pub fn is_en(&self) -> bool {
    self.language_code.starts_with("en")
  }

  /// Checks if the language code is for Spanish.
  /// This method checks for the primary 'es' subtag.
  #[must_use]
  pub fn is_es(&self) -> bool {
    self.language_code.starts_with("es")
  }

  /// Checks if the language code is for French.
  /// This method checks for the primary 'fr' subtag.
  #[must_use]
  pub fn is_fr(&self) -> bool {
    self.language_code.starts_with("fr")
  }

  /// Checks if the language code is for German.
  /// This method checks for the primary 'de' subtag.
  #[must_use]
  pub fn is_de(&self) -> bool {
    self.language_code.starts_with("de")
  }

  /// Checks if the language code is for Simplified Chinese (zh-Hans).
  /// This method specifically looks for "zh-Hans".
  #[must_use]
  pub fn is_zh_hans(&self) -> bool {
    self.language_code == "zh-Hans"
  }

  /// Checks if the language code is for Traditional Chinese (zh-Hant).
  /// This method specifically looks for "zh-Hant".
  #[must_use]
  pub fn is_zh_hant(&self) -> bool {
    self.language_code == "zh-Hant"
  }

  /// Checks if the language code is for Hindi.
  /// This method checks for the primary 'hi' subtag.
  #[must_use]
  pub fn is_hi(&self) -> bool {
    self.language_code.starts_with("hi")
  }

  /// Checks if the language code is for Portuguese.
  /// This method checks for the primary 'pt' subtag.
  #[must_use]
  pub fn is_pt(&self) -> bool {
    self.language_code.starts_with("pt")
  }

  /// Checks if the language code is for Russian.
  /// This method checks for the primary 'ru' subtag.
  #[must_use]
  pub fn is_ru(&self) -> bool {
    self.language_code.starts_with("ru")
  }

  /// Checks if the language code is for Japanese.
  /// This method checks for the primary 'ja' subtag.
  #[must_use]
  pub fn is_ja(&self) -> bool {
    self.language_code.starts_with("ja")
  }

  /// Checks if the language code is for Arabic.
  /// This method checks for the primary 'ar' subtag.
  #[must_use]
  pub fn is_ar(&self) -> bool {
    self.language_code.starts_with("ar")
  }

  /// Checks if the language code is for Italian.
  /// This method checks for the primary 'it' subtag.
  #[must_use]
  pub fn is_it(&self) -> bool {
    self.language_code.starts_with("it")
  }
}
