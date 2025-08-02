use std::fmt::Debug;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::static_data::{
    base_violations::get_base_violations_path, gt_rules::get_gt_rule_path,
    lt_rules::get_lt_rule_path, lte_rules::get_lte_rule_path,
  },
};

pub fn lt<T>(field_context: &FieldContext, value: &T, target: &T) -> Result<(), Violation>
where
  T: PartialOrd + Debug,
{
  let check = *value < *target;

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(field_context.field_data.proto_type as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.key_type.map(|t| t as i32),
      value_type: field_context.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript.clone(),
    };

    elements.push(current_elem);

    let mut violation_elements = get_base_violations_path(&field_context.field_data.kind);

    let (type_name, violation_path) = get_lt_rule_path(&field_context.field_data.proto_type);

    violation_elements.extend(violation_path);

    let violation = Violation {
      rule_id: Some(format!("{}.lt", type_name)),
      message: Some(format!(
        "{} must be less than {:?}",
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

pub fn lte<T>(field_context: &FieldContext, value: &T, target: &T) -> Result<(), Violation>
where
  T: PartialOrd + Debug,
{
  let check = *value <= *target;

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(field_context.field_data.proto_type as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.key_type.map(|t| t as i32),
      value_type: field_context.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript.clone(),
    };

    elements.push(current_elem);

    let mut violation_elements = get_base_violations_path(&field_context.field_data.kind);

    let (type_name, violation_path) = get_lte_rule_path(&field_context.field_data.proto_type);

    violation_elements.extend(violation_path);

    let violation = Violation {
      rule_id: Some(format!("{}.lte", type_name)),
      message: Some(format!(
        "{} must be less than or equal to {:?}",
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

pub fn gt<T>(field_context: &FieldContext, value: &T, target: &T) -> Result<(), Violation>
where
  T: PartialOrd + Debug,
{
  let check = *value > *target;

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(field_context.field_data.proto_type as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.key_type.map(|t| t as i32),
      value_type: field_context.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript.clone(),
    };

    elements.push(current_elem);

    let mut violation_elements = get_base_violations_path(&field_context.field_data.kind);

    let (type_name, violation_path) = get_gt_rule_path(&field_context.field_data.proto_type);

    violation_elements.extend(violation_path);

    let violation = Violation {
      rule_id: Some(format!("{}.gt", type_name)),
      message: Some(format!(
        "{} must be greater than {:?}",
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

pub fn gte<T>(field_context: &FieldContext, value: &T, target: &T) -> Result<(), Violation>
where
  T: PartialOrd + Debug,
{
  let check = *value >= *target;

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(field_context.field_data.proto_type as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.key_type.map(|t| t as i32),
      value_type: field_context.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript.clone(),
    };

    elements.push(current_elem);

    let mut violation_elements = get_base_violations_path(&field_context.field_data.kind);

    let (type_name, violation_path) = get_lte_rule_path(&field_context.field_data.proto_type);

    violation_elements.extend(violation_path);

    let violation = Violation {
      rule_id: Some(format!("{}.gte", type_name)),
      message: Some(format!(
        "{} must be greater than or equal to {:?}",
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
