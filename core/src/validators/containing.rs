use std::fmt::Debug;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::{
    common::get_base_violations_path,
    static_data::{in_rules::get_in_rule_path, not_in_rules::get_not_in_rule_path},
  },
};

pub fn in_list<T>(
  field_context: FieldContext,
  value: Option<T>,
  target: &[T],
) -> Result<(), Violation>
where
  T: PartialEq + Debug,
{
  let check = match value {
    Some(val) => target.contains(&val),
    None => return Ok(()),
  };

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(field_context.field_data.proto_type as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.field_data.key_type.map(|t| t as i32),
      value_type: field_context.field_data.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript,
    };

    elements.push(current_elem);

    let mut violation_elements = get_base_violations_path(&field_context.field_data.kind);

    let (type_name, violation_path) = get_in_rule_path(&field_context.field_data.proto_type);

    violation_elements.extend(violation_path);

    let violation = Violation {
      rule_id: Some(format!("{}.in", type_name)),
      message: Some(format!(
        "{} has to be one of these values: {:?}",
        field_context.field_data.proto_name.clone(),
        target
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn not_in_list<T>(
  field_context: FieldContext,
  value: Option<T>,
  target: &[T],
) -> Result<(), Violation>
where
  T: PartialEq + Debug,
{
  let check = match value {
    Some(val) => !target.contains(&val),
    None => return Ok(()),
  };

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(field_context.field_data.proto_type as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.field_data.key_type.map(|t| t as i32),
      value_type: field_context.field_data.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript,
    };

    elements.push(current_elem);

    let mut violation_elements = get_base_violations_path(&field_context.field_data.kind);

    let (type_name, violation_path) = get_not_in_rule_path(&field_context.field_data.proto_type);

    violation_elements.extend(violation_path);

    let violation = Violation {
      rule_id: Some(format!("{}.not_in", type_name)),
      message: Some(format!(
        "{} cannot be one of these values: {:?}",
        field_context.field_data.proto_name.clone(),
        target
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}
