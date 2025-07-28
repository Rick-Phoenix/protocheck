use cel_interpreter::{Context, Program, Value};

use crate::{
  field_data::{FieldContext, FieldData},
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::common::get_base_violations_path,
  ProtoType,
};

#[derive(Debug, Clone)]
pub struct CelRuleData {
  pub rule_id: String,
  pub error_message: String,
  pub is_for_message: bool,
  pub validation_type: String,
}

pub fn validate_cel(
  field_context: FieldContext,
  program: &'static Program,
  cel_context: Context,
  rule_data: &CelRuleData,
) -> Result<(), Violation> {
  let result = program.execute(&cel_context);

  let CelRuleData {
    validation_type: validator_type,
    is_for_message,
    rule_id,
    error_message,
  } = rule_data;

  let error_prefix = format!(
    "Error during Cel validation for {} {}:",
    validator_type, field_context.field_data.proto_name
  );

  match result {
    Ok(value) => {
      if let Value::Bool(bool_value) = value {
        if bool_value {
          Ok(())
        } else {
          let mut elements = field_context.parent_elements.to_vec();
          let mut violations_path = vec![];
          if !is_for_message {
            let current_elem = FieldPathElement {
              field_type: Some(field_context.field_data.proto_type as i32),
              field_name: Some(field_context.field_data.proto_name),
              field_number: Some(field_context.field_data.tag as i32),
              key_type: field_context.field_data.key_type.map(|t| t as i32),
              value_type: field_context.field_data.value_type.map(|t| t as i32),
              subscript: field_context.subscript,
            };
            elements.push(current_elem);

            let FieldData {
              is_repeated_item,
              is_map_key,
              is_map_value,
              ..
            } = field_context.field_data;

            violations_path.extend(get_base_violations_path(
              is_repeated_item,
              is_map_key,
              is_map_value,
            ));

            violations_path.push(FieldPathElement {
              field_name: Some("cel".to_string()),
              field_number: Some(23),
              field_type: Some(ProtoType::Message as i32),
              key_type: None,
              value_type: None,
              subscript: None,
            });
          }

          Err(Violation {
            message: Some(error_message.to_string()),
            rule_id: Some(rule_id.clone()),
            rule: Some(FieldPath {
              elements: violations_path,
            }),
            field: Some(FieldPath { elements }),
            for_key: Some(field_context.field_data.is_map_key),
          })
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
