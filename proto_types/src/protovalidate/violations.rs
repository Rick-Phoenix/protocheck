use std::fmt::Write;

use crate::protovalidate::{FieldPath, FieldPathElement, Violation, Violations};

impl FieldPath {
  pub fn last_field(&self) -> Option<&FieldPathElement> {
    if let Some(last_field) = self.elements.last() {
      return Some(last_field);
    }
    None
  }

  pub fn parent_field(&self) -> Option<&FieldPathElement> {
    let second_last = self.elements.get(self.elements.len().wrapping_sub(2));

    match second_last {
      Some(el) => Some(el),
      None => None,
    }
  }

  pub fn has_field(&self) -> bool {
    self.last_field().is_some()
  }

  pub fn field_name(&self) -> Option<&str> {
    self.last_field().map(|f| f.field_name())
  }

  pub fn get_field(&self, name: &str) -> Option<&FieldPathElement> {
    self
      .elements
      .iter()
      .find(|&field| field.field_name() == name)
  }

  pub fn field_path(&self) -> String {
    let mut path = String::new();

    for (idx, field) in self.elements.iter().enumerate() {
      path.push_str(field.field_name());

      if let Some(key) = &field.subscript {
        path.push('.');
        write!(path, "{}", key).unwrap();
      }

      if idx != self.elements.len() - 1 {
        path.push('.');
      }
    }

    path
  }
}

impl Violations {
  pub fn violation_by_rule_id(&self, rule_id: &str) -> Option<&Violation> {
    self.violations.iter().find(|v| v.rule_id() == rule_id)
  }
}

impl Violation {
  pub fn last_field(&self) -> Option<&FieldPathElement> {
    if let Some(fields) = &self.field {
      return fields.last_field();
    }

    None
  }

  pub fn parent_field(&self) -> Option<&FieldPathElement> {
    if let Some(fields) = &self.field {
      return fields.parent_field();
    }

    None
  }

  pub fn get_field(&self, name: &str) -> Option<&FieldPathElement> {
    if let Some(fields) = &self.field {
      return fields.get_field(name);
    }

    None
  }

  pub fn field_path(&self) -> Option<String> {
    if let Some(fields) = &self.field {
      return Some(fields.field_path());
    }

    None
  }

  pub fn rule_path(&self) -> Option<String> {
    if let Some(rules) = &self.rule {
      return Some(rules.field_path());
    }

    None
  }

  pub fn has_field(&self) -> bool {
    self.last_field().is_some()
  }

  pub fn field_name(&self) -> Option<&str> {
    self.last_field().map(|f| f.field_name())
  }
}
