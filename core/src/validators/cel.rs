use std::{sync::LazyLock, vec};

use cel::{Context, Program, Value as CelValue};
use chrono::Utc;
use proto_types::cel::CelConversionError;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::static_data::base_violations::create_violation,
  ProtoType,
};

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

  let error_prefix = format!(
    "Error during Cel validation for field {}:",
    field_context.proto_name
  );

  let mut cel_context = Context::default();
  cel_context.add_variable_from_value("now", CelValue::Timestamp(Utc::now().into()));

  cel_context.add_variable_from_value("this", value);

  let result = program.execute(&cel_context);

  match result {
    Ok(value) => {
      if let CelValue::Bool(bool_value) = value {
        if bool_value {
          Ok(())
        } else {
          Err(create_violation(
            field_context,
            &CEL_VIOLATION,
            rule_id,
            error_message,
          ))
        }
      } else {
        println!(
          "{} expected boolean result from expression, got `{:?}`",
          error_prefix,
          value.type_of()
        );
        Err(create_violation(
          field_context,
          &CEL_VIOLATION,
          "internal server error",
          "internal server error",
        ))
      }
    }
    Err(e) => {
      println!("{} {:?}", error_prefix, e);
      Err(create_violation(
        field_context,
        &CEL_VIOLATION,
        "internal server error",
        "internal server error",
      ))
    }
  }
}

pub fn validate_cel_field_try_into<T>(
  field_context: &FieldContext,
  rule: CelRule,
  value: T,
) -> Result<(), Violation>
where
  T: TryInto<CelValue, Error = CelConversionError> + Clone,
{
  let cel_conversion: Result<CelValue, CelConversionError> = value.try_into();

  match cel_conversion {
    Ok(cel_val) => validate_cel_field_with_val(field_context, rule, cel_val),
    Err(e) => {
      println!(
        "Failed to convert field {} to Cel value: {}",
        rule.item_full_name, e
      );

      Err(create_violation(
        field_context,
        &CEL_VIOLATION,
        "internal server error",
        "internal server error",
      ))
    }
  }
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

  let error_prefix = format!("Error during Cel validation for message {}:", message_name);

  let mut cel_context = Context::default();
  cel_context.add_variable_from_value("now", CelValue::Timestamp(Utc::now().into()));

  let cel_conversion: Result<CelValue, CelConversionError> = value.try_into();

  match cel_conversion {
    Ok(cel_val) => {
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
            println!(
              "{} expected boolean result from expression, got `{:?}`",
              error_prefix,
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
          println!("{} program failed to compile: {:?}", error_prefix, e);
          Err(create_cel_message_violation(
            "internal_server_error",
            "internal server error",
            parent_elements,
          ))
        }
      }
    }
    Err(e) => {
      println!(
        "{} could not convert message to Cel value: {:?}",
        error_prefix, e
      );
      Err(create_cel_message_violation(
        "internal_server_error",
        "internal server error",
        parent_elements,
      ))
    }
  }
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
      elements: CEL_VIOLATION.clone(),
    }),
    field: field_path,
    for_key: None,
  }
}

static CEL_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![FieldPathElement {
    field_name: Some("cel".to_string()),
    field_number: Some(23),
    field_type: Some(ProtoType::Message as i32),
    key_type: None,
    value_type: None,
    subscript: None,
  }]
});
