use std::{collections::HashSet, fmt::Debug, hash::Hash};

use prost::bytes::Bytes;
use proto_types::Any;

use crate::{
  field_data::FieldContext,
  protovalidate::Violation,
  validators::static_data::{
    base_violations::create_violation, in_rules::get_in_rule_path,
    not_in_rules::get_not_in_rule_path,
  },
};

pub fn bytes_in_list(
  field_context: &FieldContext,
  value: &Bytes,
  target: &'static HashSet<Bytes>,
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = target.contains(value);
  if check {
    Ok(())
  } else {
    Err(create_in_list_violation(field_context, error_message))
  }
}

pub fn bytes_not_in_list(
  field_context: &FieldContext,
  value: &Bytes,
  target: &'static HashSet<Bytes>,
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = !target.contains(value);
  if check {
    Ok(())
  } else {
    Err(create_not_in_list_violation(field_context, error_message))
  }
}

pub fn string_in_list(
  field_context: &FieldContext,
  value: &str,
  target: &'static HashSet<&'static str>,
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = target.contains(value);
  if check {
    Ok(())
  } else {
    Err(create_in_list_violation(field_context, error_message))
  }
}

pub fn string_not_in_list(
  field_context: &FieldContext,
  value: &str,
  target: &'static HashSet<&'static str>,
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = !target.contains(value);
  if check {
    Ok(())
  } else {
    Err(create_not_in_list_violation(field_context, error_message))
  }
}

pub fn any_in_list(
  field_context: &FieldContext,
  value: &Any,
  target: &'static HashSet<&'static str>,
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = target.contains(value.type_url.as_str());
  if check {
    Ok(())
  } else {
    Err(create_in_list_violation(field_context, error_message))
  }
}

pub fn any_not_in_list(
  field_context: &FieldContext,
  value: &Any,
  target: &'static HashSet<&'static str>,
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = !target.contains(value.type_url.as_str());
  if check {
    Ok(())
  } else {
    Err(create_not_in_list_violation(field_context, error_message))
  }
}

fn create_in_list_violation(field_context: &FieldContext, error_message: &str) -> Violation {
  let (type_name, violation_path) = get_in_rule_path(field_context.field_kind.inner_type())
    .expect("Could not find 'in_list' rule path");
  let rule_id = format!("{}.in", type_name);

  create_violation(field_context, violation_path, &rule_id, error_message)
}

pub fn in_list<T>(
  field_context: &FieldContext,
  value: T,
  target: &'static HashSet<T>,
  error_message: &'static str,
) -> Result<(), Violation>
where
  T: Eq + Debug + Hash,
{
  let check = target.contains(&value);

  if check {
    Ok(())
  } else {
    Err(create_in_list_violation(field_context, error_message))
  }
}

pub fn not_in_list<T>(
  field_context: &FieldContext,
  value: T,
  target: &'static HashSet<T>,
  error_message: &'static str,
) -> Result<(), Violation>
where
  T: Eq + Hash + Debug,
{
  let check = !target.contains(&value);

  if check {
    Ok(())
  } else {
    Err(create_not_in_list_violation(field_context, error_message))
  }
}

fn create_not_in_list_violation(field_context: &FieldContext, error_message: &str) -> Violation {
  let (type_name, violation_path) = get_not_in_rule_path(field_context.field_kind.inner_type())
    .expect("Could not find 'not_in_list' rule path");
  let rule_id = format!("{}.not_in", type_name);

  create_violation(field_context, violation_path, &rule_id, error_message)
}
