use std::fmt::Debug;

use proto_types::Any;

use crate::{
  field_data::FieldContext,
  protovalidate::Violation,
  validators::static_data::{
    base_violations::create_violation, in_rules::get_in_rule_path,
    not_in_rules::get_not_in_rule_path,
  },
};

pub fn string_in_list(
  field_context: &FieldContext,
  value: &str,
  target: &'static [&'static str],
) -> Result<(), Violation> {
  let check = target.contains(&value);
  if check {
    Ok(())
  } else {
    let values_list_string = format!("[{}]", target.join(", "));
    Err(create_in_list_violation(field_context, &values_list_string))
  }
}

pub fn string_not_in_list(
  field_context: &FieldContext,
  value: &str,
  target: &'static [&'static str],
) -> Result<(), Violation> {
  let check = !target.contains(&value);
  if check {
    Ok(())
  } else {
    let values_list_string = format!("[{}]", target.join(", "));
    Err(create_in_list_violation(field_context, &values_list_string))
  }
}

pub fn any_in_list(
  field_context: &FieldContext,
  value: &Any,
  target: &'static [&'static str],
) -> Result<(), Violation> {
  let check = target.contains(&value.type_url.as_str());
  if check {
    Ok(())
  } else {
    let values_list_string = format!("(type_url) [{}]", target.join(", "));
    Err(create_in_list_violation(field_context, &values_list_string))
  }
}

pub fn any_not_in_list(
  field_context: &FieldContext,
  value: &Any,
  target: &'static [&'static str],
) -> Result<(), Violation> {
  let check = !target.contains(&value.type_url.as_str());
  if check {
    Ok(())
  } else {
    let values_list_string = format!("(type_url) [{}]", target.join(", "));
    Err(create_in_list_violation(field_context, &values_list_string))
  }
}

fn create_in_list_violation(field_context: &FieldContext, values_list_string: &str) -> Violation {
  let (type_name, violation_path) = get_in_rule_path(field_context.field_kind.inner_type())
    .expect("Could not find 'in_list' rule path");
  let rule_id = format!("{}.in", type_name);
  let error_message = format!("has to be one of these values: {}", values_list_string);

  create_violation(field_context, violation_path, &rule_id, &error_message)
}

pub fn in_list<T>(
  field_context: &FieldContext,
  value: T,
  target: &'static [T],
) -> Result<(), Violation>
where
  T: PartialEq + Debug,
{
  let check = target.contains(&value);

  if check {
    Ok(())
  } else {
    let values_list_string = format!("{:?}", target);
    Err(create_in_list_violation(field_context, &values_list_string))
  }
}

pub fn not_in_list<T>(
  field_context: &FieldContext,
  value: T,
  target: &'static [T],
) -> Result<(), Violation>
where
  T: PartialEq + Debug,
{
  let check = !target.contains(&value);

  if check {
    Ok(())
  } else {
    let values_list_string = format!("{:?}", target);
    Err(create_not_in_list_violation(
      field_context,
      &values_list_string,
    ))
  }
}

fn create_not_in_list_violation(
  field_context: &FieldContext,
  values_list_string: &str,
) -> Violation {
  let (type_name, violation_path) = get_not_in_rule_path(field_context.field_kind.inner_type())
    .expect("Could not find 'not_in_list' rule path");
  let rule_id = format!("{}.not_in", type_name);
  let error_message = format!("cannot be one of these values: {}", values_list_string);

  create_violation(field_context, violation_path, &rule_id, &error_message)
}
