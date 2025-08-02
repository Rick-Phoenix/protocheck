use cel_interpreter::{Context, Program, Value as CelValue};
use chrono::{DateTime, FixedOffset, Utc};
use proto_types::{Duration, DurationError, Timestamp, TimestampError};
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

#[derive(Debug)]
pub enum CelFieldKind<'a>
// where
//   T: serde::Serialize,
{
  Duration(Option<&'a Duration>),
  Timestamp(&'a Timestamp),
  // Other(&'a T),
}

#[derive(Debug)]
pub struct CelField<'a> {
  pub rule_id: String,
  pub error_message: String,
  pub field_context: &'a FieldContext<'a>,
}

pub fn validate_cel_field(
  cel_field_data: &CelField<'_>,
  program: &'static Program,
  value: Option<&CelFieldKind<'_>>,
) -> Result<(), Violation>
// where
//   T: serde::Serialize,
{
  if value.is_none() {
    return Ok(());
  }

  let CelField {
    rule_id,
    error_message,
    field_context,
    ..
  } = cel_field_data;

  let error_prefix = format!(
    "Error during Cel validation for field {}:",
    field_context.field_data.proto_name
  );

  let unwrapped_val = value.unwrap();
  let mut cel_context = Context::default();
  cel_context.add_variable_from_value("now", CelValue::Timestamp(Utc::now().into()));

  match unwrapped_val {
    &CelFieldKind::Duration(duration) => {
      let chrono_duration: chrono::Duration =
        duration
          .unwrap()
          .to_owned()
          .try_into()
          .map_err(|e: DurationError| {
            create_cel_violation("invalid_cel_data".to_string(), e.to_string(), field_context)
          })?;

      let cel_duration: CelValue = CelValue::Duration(chrono_duration);

      cel_context.add_variable_from_value("this", cel_duration);
    }
    &CelFieldKind::Timestamp(timestamp) => {
      let timestamp_val: DateTime<FixedOffset> =
        timestamp
          .to_owned()
          .try_into()
          .map_err(|e: TimestampError| {
            create_cel_violation("invalid_cel_data".to_string(), e.to_string(), field_context)
          })?;
      let cel_timestamp: CelValue = CelValue::Timestamp(timestamp_val);

      cel_context.add_variable_from_value("this", cel_timestamp);
    } // CelFieldKind::Other(value) => {
      //   cel_context.add_variable("this", value).map_err(|e| {
      //     create_cel_violation("invalid_cel_data".to_string(), e.to_string(), field_context)
      //   })?;
      // }
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
