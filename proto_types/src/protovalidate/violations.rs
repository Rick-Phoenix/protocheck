use crate::protovalidate::{FieldPath, FieldPathElement, Violation, Violations};

impl FieldPath {
  /// Returns the last member in the elements list, if the list is not empty.
  pub fn last_field(&self) -> Option<&FieldPathElement> {
    if let Some(last_field) = self.elements.last() {
      return Some(last_field);
    }
    None
  }

  /// Returns the second last member in the elements list, if the list is not empty.
  pub fn parent_field(&self) -> Option<&FieldPathElement> {
    let second_last = self.elements.get(self.elements.len().wrapping_sub(2));

    match second_last {
      Some(el) => Some(el),
      None => None,
    }
  }

  /// Checks if the elements list is empty or not.
  pub fn has_fields(&self) -> bool {
    self.last_field().is_some()
  }

  /// Returns the name of the last member in the elements list, if there is one.
  pub fn last_field_name(&self) -> Option<&str> {
    self.last_field().map(|f| f.field_name())
  }

  /// Searches for a FieldPathElement by name in the elements list.
  pub fn get_field(&self, name: &str) -> Option<&FieldPathElement> {
    self
      .elements
      .iter()
      .find(|&field| field.field_name() == name)
  }

  /// Returns a vector with the names from each element in the list.
  pub fn field_path(&self) -> Vec<String> {
    let mut path: Vec<String> = Vec::new();

    for field in self.elements.iter() {
      path.push(field.field_name().to_string());

      if let Some(key) = &field.subscript {
        path.push(key.to_string());
      }
    }

    path
  }

  /// Returns all of the names from each path element, joined by a dot (e.g. `person.friend.0.address.street_name`)
  pub fn field_path_str(&self) -> String {
    self.field_path().join(".")
  }
}

impl Violations {
  /// Searches for a violation with a specific rule id.
  pub fn violation_by_rule_id(&self, rule_id: &str) -> Option<&Violation> {
    self.violations.iter().find(|v| v.rule_id() == rule_id)
  }
}

impl Violation {
  /// Returns the last member in the elements list, if there is one.
  pub fn last_field(&self) -> Option<&FieldPathElement> {
    if let Some(fields) = &self.field {
      return fields.last_field();
    }

    None
  }

  /// Returns the second last member in the elements list, if there is one.
  pub fn parent_field(&self) -> Option<&FieldPathElement> {
    if let Some(fields) = &self.field {
      return fields.parent_field();
    }

    None
  }

  /// Searches for a field in the FieldPath list with a specific name.
  pub fn get_field(&self, name: &str) -> Option<&FieldPathElement> {
    if let Some(fields) = &self.field {
      return fields.get_field(name);
    }

    None
  }

  /// If the FieldPath is present, it will return the list of the names for each path element.
  pub fn field_path(&self) -> Option<Vec<String>> {
    if let Some(fields) = &self.field {
      return Some(fields.field_path());
    }

    None
  }

  /// Returns the element names composing the violation's rule, like ["string", "max_len"].
  pub fn rule_path(&self) -> Option<Vec<String>> {
    if let Some(rules) = &self.rule {
      return Some(rules.field_path());
    }

    None
  }

  /// If there is a FieldPath, it returns the path elements' names, joined by a dot (e.g. `person.friend.0.address.street_name`).
  pub fn field_path_str(&self) -> Option<String> {
    if let Some(fields) = &self.field {
      return Some(fields.field_path_str());
    }

    None
  }

  /// If a rule path is defined, it returns the rule path segments for this violation, joined by a dot (e.g. `map.keys.string.min_len`)
  pub fn rule_path_str(&self) -> Option<String> {
    if let Some(rules) = &self.rule {
      return Some(rules.field_path_str());
    }

    None
  }

  /// Checks whether this violation has a FieldPath or not. This may not be the case when a violation is triggered by a rule defined with (buf.validate.message).cel in a message
  pub fn has_fields(&self) -> bool {
    self.last_field().is_some()
  }

  /// Checks if the list of FieldPathElements contains a field with a particular name.
  pub fn has_field_by_name(&self, name: &str) -> bool {
    self.get_field(name).is_some()
  }

  /// If a list of path elements is defined, it returns the name of the invalid field (the last field in the list of path elements)
  pub fn field_name(&self) -> Option<&str> {
    self.last_field().map(|f| f.field_name())
  }
}
