use std::fmt::Debug;

use crate::{
  field_data::FieldContext,
  protovalidate::Violation,
  validators::static_data::{base_violations::create_violation, const_rules::get_const_rule_path},
};

pub fn constant<T>(
  field_context: &FieldContext,
  value: T,
  target: T,
  error_message: &'static str,
) -> Result<(), Violation>
where
  T: PartialEq + Debug,
{
  let check = value == target;

  if check {
    Ok(())
  } else {
    let (type_name, const_violation) = get_const_rule_path(field_context.field_kind.inner_type())
      .expect("Could not find 'const' rule path");
    let rule_id = format!("{}.const", type_name);

    Err(create_violation(
      field_context,
      const_violation,
      &rule_id,
      error_message,
    ))
  }
}
