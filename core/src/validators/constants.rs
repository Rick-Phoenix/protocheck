use std::fmt::Debug;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::static_data::{
    base_violations::get_base_violations_path, const_rules::get_const_rule_path,
  },
};

pub fn constant<T>(field_context: &FieldContext, value: &T, target: &T) -> Result<(), Violation>
where
  T: PartialEq + Debug,
{
  let check = *value == *target;

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(field_context.field_kind.inner_type().into()),
      field_name: Some(field_context.proto_name.to_string()),
      key_type: field_context.key_type.map(|t| t as i32),
      value_type: field_context.value_type.map(|t| t as i32),
      field_number: Some(field_context.tag as i32),
      subscript: field_context.subscript.clone(),
    };

    elements.push(current_elem);

    let mut violation_elements = get_base_violations_path(field_context.field_kind);

    let (type_name, const_violation) = get_const_rule_path(field_context.field_kind.inner_type());

    violation_elements.extend(const_violation);

    let violation = Violation {
      rule_id: Some(format!("{}.const", type_name)),
      message: Some(format!(
        "{} has to be equal to {:?}",
        field_context.proto_name.clone(),
        target
      )),
      for_key: field_context.field_kind.is_map_key().then_some(true),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}
