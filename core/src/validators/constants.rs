use std::fmt::Debug;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::static_data::{
    base_violations::get_base_violations_path, const_rules::get_const_rule_path,
  },
};

pub fn constant<T>(
  field_context: FieldContext,
  value: Option<T>,
  target: T,
) -> Result<(), Violation>
where
  T: PartialEq + Debug,
{
  let check = match value {
    Some(val) => val == target,
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

    let (type_name, const_violation) = get_const_rule_path(&field_context.field_data.proto_type);

    violation_elements.extend(const_violation);

    let violation = Violation {
      rule_id: Some(format!("{}.const", type_name)),
      message: Some(format!(
        "{} has to be equal to {:?}",
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
