use std::vec;

use cel_interpreter::{Context, Program, Value as CelValue};
use chrono::Utc;
use proto_types::{DurationError, TimestampError};
use thiserror::Error;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::static_data::base_violations::get_base_violations_path,
  ProtoType,
};

#[derive(Debug, Error)]
pub enum CelConversionError {
  #[error("{0}")]
  DurationError(#[from] DurationError),

  #[error("{0}")]
  TimestampError(#[from] TimestampError),
}

pub struct CelRule {
  pub id: String,
  pub error_message: String,
  pub program: &'static Program,
}

pub fn validate_cel_field_with_val<T>(
  field_context: &FieldContext,
  rule: &CelRule,
  value: &T,
) -> Result<(), Violation>
where
  T: Into<CelValue> + Clone,
{
  let CelRule {
    id: rule_id,
    error_message,
    program,
    ..
  } = rule;

  let error_prefix = format!(
    "Error during Cel validation for field {}:",
    field_context.field_data.proto_name
  );

  let mut cel_context = Context::default();
  cel_context.add_variable_from_value("now", CelValue::Timestamp(Utc::now().into()));

  cel_context.add_variable_from_value("this", value.clone().into());

  let result = program.execute(&cel_context);

  match result {
    Ok(value) => {
      if let CelValue::Bool(bool_value) = value {
        if bool_value {
          Ok(())
        } else {
          Err(create_cel_violation(
            rule_id.to_string(),
            error_message.to_string(),
            field_context,
          ))
        }
      } else {
        println!(
          "{} expected boolean result from expression, got `{:?}`",
          error_prefix,
          value.type_of()
        );
        Err(Violation {
          message: Some("Internal server error".to_string()),
          rule_id: Some("internal_server_error".to_string()),
          rule: None,
          field: None,
          for_key: Some(false),
        })
      }
    }
    Err(e) => {
      println!("{} {:?}", error_prefix, e);
      Err(Violation {
        message: Some("Internal server error".to_string()),
        rule_id: Some("internal_server_error".to_string()),
        rule: None,
        field: None,
        for_key: Some(false),
      })
    }
  }
}

pub fn validate_cel_field<T>(
  field_context: &FieldContext,
  rule: &CelRule,
  value: &T,
) -> Result<(), Violation>
where
  T: TryInto<CelValue> + Clone,
  CelConversionError: From<<T as TryInto<CelValue>>::Error>,
{
  let CelRule {
    id: rule_id,
    error_message,
    program,
    ..
  } = rule;

  let error_prefix = format!(
    "Error during Cel validation for field {}:",
    field_context.field_data.proto_name
  );

  let mut cel_context = Context::default();
  cel_context.add_variable_from_value("now", CelValue::Timestamp(Utc::now().into()));

  let cel_conversion: Result<CelValue, CelConversionError> =
    value.clone().try_into().map_err(|e| e.into());

  match cel_conversion {
    Ok(cel_val) => {
      cel_context.add_variable_from_value("this", cel_val);
    }
    Err(e) => {
      return Err(create_cel_violation(
        rule_id.to_string(),
        e.to_string(),
        field_context,
      ));
    }
  };

  let result = program.execute(&cel_context);

  match result {
    Ok(value) => {
      if let CelValue::Bool(bool_value) = value {
        if bool_value {
          Ok(())
        } else {
          Err(create_cel_violation(
            rule_id.to_string(),
            error_message.to_string(),
            field_context,
          ))
        }
      } else {
        println!(
          "{} expected boolean result from expression, got `{:?}`",
          error_prefix,
          value.type_of()
        );
        Err(Violation {
          message: Some("Internal server error".to_string()),
          rule_id: Some("internal_server_error".to_string()),
          rule: None,
          field: None,
          for_key: Some(false),
        })
      }
    }
    Err(e) => {
      println!("{} {:?}", error_prefix, e);
      Err(Violation {
        message: Some("Internal server error".to_string()),
        rule_id: Some("internal_server_error".to_string()),
        rule: None,
        field: None,
        for_key: Some(false),
      })
    }
  }
}

pub fn validate_cel_message<T>(
  message_name: &str,
  parent_elements: &[FieldPathElement],
  rule: &CelRule,
  value: &T,
) -> Result<(), Violation>
where
  T: TryInto<CelValue, Error = CelConversionError> + Clone,
{
  let CelRule {
    id: rule_id,
    error_message,
    program,
  } = rule;

  let error_prefix = format!("Error during Cel validation for field {}:", message_name);

  let mut cel_context = Context::default();
  cel_context.add_variable_from_value("now", CelValue::Timestamp(Utc::now().into()));

  let cel_conversion: Result<CelValue, CelConversionError> = value.clone().try_into();

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
            Err(Violation {
              message: Some("Internal server error".to_string()),
              rule_id: Some("internal_server_error".to_string()),
              rule: None,
              field: None,
              for_key: Some(false),
            })
          }
        }
        Err(e) => {
          println!("{} {:?}", error_prefix, e);
          Err(Violation {
            message: Some("Internal server error".to_string()),
            rule_id: Some("internal_server_error".to_string()),
            rule: None,
            field: None,
            for_key: Some(false),
          })
        }
      }
    }
    Err(e) => Err(create_cel_message_violation(
      rule_id,
      &e.to_string(),
      parent_elements,
    )),
  }
}

pub fn create_cel_message_violation(
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
      elements: vec![FieldPathElement {
        field_name: Some("cel".to_string()),
        field_number: Some(23),
        field_type: Some(ProtoType::Message as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      }],
    }),
    field: field_path,
    for_key: None,
  }
}

fn create_cel_violation(
  rule_id: String,
  error_message: String,
  field_context: &FieldContext<'_>,
) -> Violation {
  let cel_violation = FieldPathElement {
    field_name: Some("cel".to_string()),
    field_number: Some(23),
    field_type: Some(ProtoType::Message as i32),
    key_type: None,
    value_type: None,
    subscript: None,
  };

  let mut elements = field_context.parent_elements.to_vec();

  let current_elem = FieldPathElement {
    field_type: Some(field_context.field_data.proto_type as i32),
    field_name: Some(field_context.field_data.proto_name.clone()),
    field_number: Some(field_context.field_data.tag as i32),
    key_type: field_context.field_data.key_type.map(|t| t as i32),
    value_type: field_context.field_data.value_type.map(|t| t as i32),
    subscript: field_context.subscript.clone(),
  };
  elements.push(current_elem);

  let mut violations_path = get_base_violations_path(&field_context.field_data.kind);
  violations_path.push(cel_violation);

  Violation {
    message: Some(error_message.to_string()),
    rule_id: Some(rule_id.clone()),
    rule: Some(FieldPath {
      elements: violations_path,
    }),
    field: Some(FieldPath { elements }),
    for_key: Some(field_context.field_data.kind.is_map_key()),
  }
}
