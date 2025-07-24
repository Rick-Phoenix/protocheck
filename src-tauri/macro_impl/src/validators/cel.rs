use cel_interpreter::{Context, Program, Value};
use proto_types::{
  buf::validate::{FieldPath, FieldPathElement, Violation},
  FieldContext,
};

pub fn validate_cel(
  field_context: FieldContext,
  program: &'static Program,
  cel_context: Context,
  message: String,
  rule_id: String,
  is_for_message: bool,
) -> Result<(), Violation> {
  let result = program.execute(&cel_context);

  match result {
    Ok(value) => {
      if let Value::Bool(bool_value) = value {
        if bool_value {
          Ok(())
        } else {
          let mut elements = field_context.parent_elements.to_vec();
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
          }

          Err(Violation {
            message: Some(message),
            rule_id: Some(rule_id),
            rule: None,
            field: Some(FieldPath { elements }),
            for_key: Some(field_context.field_data.is_map_key),
          })
        }
      } else {
        println!(
          "Error during cel validation for `{}`: expected boolean result from expression, got `{:?}`",
            field_context.field_data.proto_name, value.type_of()
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
      println!(
        "Error during cel validation for `{}`: {:?}",
        field_context.field_data.proto_name, e
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
}
