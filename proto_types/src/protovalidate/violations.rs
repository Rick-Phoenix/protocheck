use prost::Message;

use crate::{
  protovalidate::{FieldPath, FieldPathElement, Violation, Violations},
  Any, Status,
};

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

  /// Returns a vector with the names from each path element (including any eventual Subscript like a vector index or map key)
  /// (e.g. `["person", "friends", "0", "address","street_name"]`)
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

  /// Returns all of the names from each path element (including any eventual Subscript like a vector index or map key), joined by a dot (e.g. `person.friends.0.address.street_name`)
  pub fn field_path_str(&self) -> String {
    self.field_path().join(".")
  }
}

impl Violations {
  /// Searches for a violation with a specific rule id.
  pub fn violation_by_rule_id(&self, rule_id: &str) -> Option<&Violation> {
    self.violations.iter().find(|v| v.rule_id() == rule_id)
  }

  /// Searches for a violation with a specific `field_path` string.
  ///
  /// Keep in mind the `field_path` will include Subscripts like vector indexes or map keys.
  ///
  /// # Examples
  /// ```rust
  /// use crate::protovalidate::{FieldPath, FieldPathElement, Violation, Violations};
  ///
  /// let violations = Violations {
  ///    violations: vec![Violation {
  ///      field: Some(FieldPath {
  ///        elements: vec![
  ///          FieldPathElement {
  ///            field_name: Some("person".to_string()),
  ///            ..Default::default()
  ///          },
  ///          FieldPathElement {
  ///            field_name: Some("name".to_string()),
  ///            ..Default::default()
  ///          },
  ///        ],
  ///      }),
  ///      ..Default::default()
  ///    }],
  ///  };

  ///  assert!(violations.violation_by_field_path("person.name").is_some());
  /// ```
  pub fn violation_by_field_path(&self, path: &str) -> Option<&Violation> {
    self.violations.iter().find(|v| {
      v.field
        .as_ref()
        .is_some_and(|vi| vi.field_path_str() == path)
    })
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

  /// If there is a FieldPath, it returns the path elements' names, joined by a dot (e.g. `person.friends.0.address.street_name`).
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
    self.field.is_some()
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

impl From<Violations> for Status {
  fn from(value: Violations) -> Self {
    let message = if value.violations.len() == 1 && !value.violations[0].message().is_empty() {
      value.violations[0].message()
    } else {
      "Validation Error"
    };

    Self {
      code: 3,
      message: message.to_string(),
      details: vec![Any {
        type_url: "type.googleapis.com/buf.validate.Violations".to_string(),
        value: value.encode_to_vec(),
      }],
    }
  }
}

impl From<Violation> for Status {
  fn from(value: Violation) -> Self {
    let message = if !value.message().is_empty() {
      value.message()
    } else {
      "Validation Error"
    };

    Self {
      code: 3,
      message: message.to_string(),
      details: vec![Any {
        type_url: "type.googleapis.com/buf.validate.Violations".to_string(),
        value: value.encode_to_vec(),
      }],
    }
  }
}
