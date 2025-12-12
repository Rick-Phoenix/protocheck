use ::cel::{Context, Program, Value as CelValue};
use chrono::Utc;
use proto_types::cel::CelConversionError;

use super::*;
use crate::protovalidate::{violations_data::CEL_VIOLATION, Violation};

pub struct CelRule {
  pub id: &'static str,
  pub error_message: &'static str,
  pub program: &'static Program,
  pub item_full_name: &'static str,
}

pub fn validate_cel_field_with_val(
  field_context: &FieldContext,
  rule: CelRule,
  value: CelValue,
) -> Result<(), Violation>
where
{
  let CelRule {
    id: rule_id,
    error_message,
    program,
    ..
  } = rule;

  let mut cel_context = Context::default();
  cel_context.add_variable_from_value("now", CelValue::Timestamp(Utc::now().into()));

  cel_context.add_variable_from_value("this", value);

  let result = program.execute(&cel_context).map_err(|e| {
    eprintln!(
      "Error during Cel validation for field {}: {e}",
      field_context.proto_name
    );

    create_cel_field_violation(rule_id, field_context, "internal server error")
  })?;

  if let CelValue::Bool(bool_value) = result {
    if bool_value {
      Ok(())
    } else {
      Err(create_cel_field_violation(
        rule_id,
        field_context,
        error_message,
      ))
    }
  } else {
    eprintln!(
      "Error during Cel validation for field {}: expected boolean result from expression, got `{:?}`",
      field_context.proto_name,
      result.type_of()
    );

    Err(create_cel_field_violation(
      rule_id,
      field_context,
      "internal server error",
    ))
  }
}

pub fn validate_cel_field_try_into<T>(
  field_context: &FieldContext,
  rule: CelRule,
  value: T,
) -> Result<(), Violation>
where
  T: TryInto<CelValue> + Clone,
  <T as std::convert::TryInto<::cel::Value>>::Error: std::fmt::Display,
{
  let cel_val: CelValue = value.try_into().map_err(|e| {
    eprintln!(
      "Failed to convert field {} to Cel value: {}",
      rule.item_full_name, e
    );

    create_cel_field_violation(rule.id, field_context, "internal server error")
  })?;

  validate_cel_field_with_val(field_context, rule, cel_val)
}

pub fn validate_cel_message<T>(
  parent_elements: &[FieldPathElement],
  rule: CelRule,
  value: T,
) -> Result<(), Violation>
where
  T: TryInto<CelValue, Error = CelConversionError>,
{
  let CelRule {
    id: rule_id,
    error_message,
    program,
    item_full_name: message_name,
  } = rule;

  let mut cel_context = Context::default();
  cel_context.add_variable_from_value("now", CelValue::Timestamp(Utc::now().into()));

  let cel_val: CelValue = value.try_into().map_err(|e| {
    eprintln!(
      "Error during Cel validation for message {message_name}: could not convert message to Cel value: {e}"
    );

    create_cel_message_violation(
      "internal_server_error",
      "internal server error",
      parent_elements,
    )
  })?;

  cel_context.add_variable_from_value("this", cel_val);
  let result = program.execute(&cel_context);

  match result {
    Ok(value) => {
      if let CelValue::Bool(bool_value) = value {
        if bool_value {
          Ok(())
        } else {
          Err(create_cel_message_violation(
            rule_id,
            error_message,
            parent_elements,
          ))
        }
      } else {
        eprintln!(
          "Error during Cel validation for message {message_name}: expected boolean result from expression, got `{:?}`",
          value.type_of()
        );

        Err(create_cel_message_violation(
          "internal_server_error",
          "internal server error",
          parent_elements,
        ))
      }
    }
    Err(e) => {
      eprintln!(
        "Error during Cel validation for message {message_name}: program failed to compile: {e}",
      );

      Err(create_cel_message_violation(
        "internal_server_error",
        "internal server error",
        parent_elements,
      ))
    }
  }
}

fn create_cel_field_violation(
  rule_id: &str,
  field_context: &FieldContext,
  error_message: &str,
) -> Violation {
  create_violation_with_custom_id(rule_id, field_context, &CEL_VIOLATION, error_message)
}

fn create_cel_message_violation(
  rule_id: &str,
  error_message: &str,
  parent_elements: &[FieldPathElement],
) -> Violation {
  let is_nested = !parent_elements.is_empty();
  let field_path = is_nested.then(|| FieldPath {
    elements: parent_elements.to_vec(),
  });

  Violation {
    message: Some(error_message.to_string()),
    rule_id: Some(rule_id.to_string()),
    rule: Some(FieldPath {
      elements: CEL_VIOLATION.elements.to_vec(),
    }),
    field: field_path,
    for_key: None,
  }
}
