use std::{collections::HashSet, hash::Hash};

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, FieldPathElement, Violation},
  ProtoType,
};

pub fn min_items<T>(
  field_context: &FieldContext,
  value: Option<&Vec<T>>,
  min_items: u64,
) -> Result<(), Violation> {
  let val = match value {
    Some(v) => v,
    None => return Ok(()),
  };

  let check = val.len() >= min_items as usize;

  if !check {
    let plural_suffix = if min_items > 1 { "s" } else { "" };

    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(field_context.field_data.proto_type as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      field_number: Some(field_context.field_data.tag as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    };
    elements.push(current_elem);

    let violation = Violation {
      rule_id: Some("repeated.min_items".to_string()),
      message: Some(format!(
        "repeated field `{}` requires at least {} item{}",
        field_context.field_data.proto_name.clone(),
        min_items,
        plural_suffix
      )),
      field: Some(FieldPath { elements }),
      for_key: None,
      rule: Some(FieldPath {
        elements: vec![
          FieldPathElement {
            field_name: Some("repeated".to_string()),
            field_number: Some(18),
            field_type: Some(ProtoType::Message as i32),
            subscript: None,
            key_type: None,
            value_type: None,
          },
          FieldPathElement {
            field_name: Some("min_items".to_string()),
            field_number: Some(1),
            field_type: Some(ProtoType::Uint64 as i32),
            key_type: None,
            value_type: None,
            subscript: None,
          },
        ],
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn max_items<T>(
  field_context: &FieldContext,
  value: Option<&Vec<T>>,
  max_items: u64,
) -> Result<(), Violation> {
  let val = match value {
    Some(v) => v,
    None => return Ok(()),
  };

  let check = val.len() <= max_items as usize;

  if !check {
    let plural_suffix = if max_items > 1 { "s" } else { "" };

    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(field_context.field_data.proto_type as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      field_number: Some(field_context.field_data.tag as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    };
    elements.push(current_elem);
    let violation = Violation {
      rule_id: Some("repeated.max_items".to_string()),
      message: Some(format!(
        "repeated field `{}` cannot have more than {} item{}",
        field_context.field_data.proto_name.clone(),
        max_items,
        plural_suffix
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: vec![
          FieldPathElement {
            field_name: Some("repeated".to_string()),
            field_number: Some(18),
            field_type: Some(ProtoType::Message as i32),
            subscript: None,
            key_type: None,
            value_type: None,
          },
          FieldPathElement {
            field_name: Some("max_items".to_string()),
            field_number: Some(2),
            field_type: Some(ProtoType::Uint64 as i32),
            key_type: None,
            value_type: None,
            subscript: None,
          },
        ],
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn unique<T>(
  field_context: &FieldContext,
  value: T,
  processed_values: &mut HashSet<T>,
) -> Result<(), Violation>
where
  T: Eq + Hash + Clone + ToString,
{
  let check = processed_values.insert(value);

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(field_context.field_data.proto_type as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript.clone(),
      key_type: None,
      value_type: None,
    };
    elements.push(current_elem);
    let violation = Violation {
      rule_id: Some("repeated.unique".to_string()),
      message: Some(format!(
        "repeated field `{}` must contain unique values",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: vec![
          FieldPathElement {
            field_name: Some("repeated".to_string()),
            field_number: Some(18),
            field_type: Some(ProtoType::Message as i32),
            subscript: None,
            key_type: None,
            value_type: None,
          },
          FieldPathElement {
            field_name: Some("unique".to_string()),
            field_number: Some(3),
            field_type: Some(ProtoType::Uint64 as i32),
            key_type: None,
            value_type: None,
            subscript: None,
          },
        ],
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub trait FloatBits {
  type Bits: Eq + std::hash::Hash;

  fn to_bits_for_unique_check(self) -> Self::Bits;
}

impl FloatBits for &f32 {
  type Bits = u32;
  fn to_bits_for_unique_check(self) -> u32 {
    self.to_bits()
  }
}

impl FloatBits for &f64 {
  type Bits = u64;
  fn to_bits_for_unique_check(self) -> u64 {
    self.to_bits()
  }
}

pub fn unique_floats<T, B>(
  field_context: &FieldContext,
  value: T,
  processed_values: &mut HashSet<B>,
) -> Result<(), Violation>
where
  T: FloatBits<Bits = B>,
  B: Eq + Hash,
{
  let bits = value.to_bits_for_unique_check();
  let check = processed_values.insert(bits);

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(field_context.field_data.proto_type as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.field_data.key_type.map(|t| t as i32),
      value_type: field_context.field_data.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript.clone(),
    };
    elements.push(current_elem);
    let violation = Violation {
      rule_id: Some("repeated.unique".to_string()),
      message: Some(format!(
        "repeated field `{}` must contain unique values",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: vec![
          FieldPathElement {
            field_name: Some("repeated".to_string()),
            field_number: Some(18),
            field_type: Some(ProtoType::Message as i32),
            subscript: None,
            key_type: None,
            value_type: None,
          },
          FieldPathElement {
            field_name: Some("unique".to_string()),
            field_number: Some(3),
            field_type: Some(ProtoType::Uint64 as i32),
            key_type: None,
            value_type: None,
            subscript: None,
          },
        ],
      }),
    };
    return Err(violation);
  };
  Ok(())
}
