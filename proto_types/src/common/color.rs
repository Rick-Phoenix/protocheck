use crate::common::Color;

impl Color {
  /// Creates a new [`Color`] instance.
  pub fn new(red: f32, green: f32, blue: f32, alpha: Option<f32>) -> Self {
    Color {
      red,
      green,
      blue,
      alpha: alpha.map(|value| crate::protobuf::FloatValue { value }),
    }
  }

  /// Checks if the values are valid.
  pub fn is_valid(&self) -> bool {
    let is_component_valid = |c: f32| (0.0..=1.0).contains(&c);
    let is_alpha_valid = self
      .alpha
      .as_ref()
      .is_none_or(|fv| fv.value >= 0.0 && fv.value <= 1.0);

    is_component_valid(self.red)
      && is_component_valid(self.green)
      && is_component_valid(self.blue)
      && is_alpha_valid
  }

  /// Returns the alpha or falls back to 1.0 as a default, as per the proto spec.
  pub fn effective_alpha(&self) -> f32 {
    self.alpha.as_ref().map_or(1.0, |fv| fv.value)
  }

  /// Converts the value to rgba8.
  pub fn to_rgba8(&self) -> (u8, u8, u8, u8) {
    let r = (self.red * 255.0).round() as u8;
    let g = (self.green * 255.0).round() as u8;
    let b = (self.blue * 255.0).round() as u8;
    let a = (self.effective_alpha() * 255.0).round() as u8;
    (r, g, b, a)
  }

  /// Converts an rgba8 color to a [`Color`].
  pub fn from_rgba8(r: u8, g: u8, b: u8, a: Option<u8>) -> Self {
    Color {
      red: r as f32 / 255.0,
      green: g as f32 / 255.0,
      blue: b as f32 / 255.0,
      alpha: a.map(|value| crate::protobuf::FloatValue {
        value: value as f32 / 255.0,
      }),
    }
  }

  /// Blends two [`Color`]s and returns the result.
  pub fn blend(&self, background_color: &Color) -> Self {
    let alpha = self.effective_alpha();
    let inverse_alpha = 1.0 - alpha;

    Color {
      red: alpha * self.red + inverse_alpha * background_color.red,
      green: alpha * self.green + inverse_alpha * background_color.green,
      blue: alpha * self.blue + inverse_alpha * background_color.blue,
      alpha: None,
    }
  }
}
