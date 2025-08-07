use std::fmt::Debug;

use crate::{
  field_data::FieldContext,
  protovalidate::Violation,
  validators::static_data::{
    base_violations::create_violation, gt_rules::get_gt_rule_path, gte_rules::get_gte_rule_path,
    lt_rules::get_lt_rule_path, lte_rules::get_lte_rule_path,
  },
};

pub fn lt<T>(field_context: &FieldContext, value: T, target: T) -> Result<(), Violation>
where
  T: PartialOrd + Debug + Copy,
{
  let check = value < target;

  if check {
    Ok(())
  } else {
    let (type_name, violation_path) = get_lt_rule_path(field_context.field_kind.inner_type())
      .expect("Could not find 'lt' rule path");
    let rule_id = format!("{}.lt", type_name);
    let error_message = format!("must be less than {:?}", target);

    Err(create_violation(
      field_context,
      violation_path,
      &rule_id,
      &error_message,
    ))
  }
}

pub fn lte<T>(field_context: &FieldContext, value: T, target: T) -> Result<(), Violation>
where
  T: PartialOrd + Debug,
{
  let check = value <= target;

  if check {
    Ok(())
  } else {
    let (type_name, violation_path) = get_lte_rule_path(field_context.field_kind.inner_type())
      .expect("Could not find 'lte' rule path");
    let rule_id = format!("{}.lte", type_name);
    let error_message = format!("cannot be greater than {:?}", target);

    Err(create_violation(
      field_context,
      violation_path,
      &rule_id,
      &error_message,
    ))
  }
}

pub fn gt<T>(field_context: &FieldContext, value: T, target: T) -> Result<(), Violation>
where
  T: PartialOrd + Debug,
{
  let check = value > target;

  if check {
    Ok(())
  } else {
    let inner = field_context.field_kind.inner_type();
    println!("INNER: {:?}", inner);
    let (type_name, violation_path) = get_gt_rule_path(field_context.field_kind.inner_type())
      .expect("Could not find 'gt' rule path");
    let rule_id = format!("{}.gt", type_name);
    let error_message = format!("must be greater than {:?}", target);

    Err(create_violation(
      field_context,
      violation_path,
      &rule_id,
      &error_message,
    ))
  }
}

pub fn gte<T>(field_context: &FieldContext, value: T, target: T) -> Result<(), Violation>
where
  T: PartialOrd + Debug,
{
  let check = value >= target;

  if check {
    Ok(())
  } else {
    let (type_name, violation_path) = get_gte_rule_path(field_context.field_kind.inner_type())
      .expect("Could not find 'gte' rule path");
    let rule_id = format!("{}.gte", type_name);
    let error_message = format!("cannot be less than {:?}", target);

    Err(create_violation(
      field_context,
      violation_path,
      &rule_id,
      &error_message,
    ))
  }
}
