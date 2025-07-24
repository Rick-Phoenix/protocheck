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
) -> Result<(), Violation> {
  let result = program.execute(&cel_context);

  match result {
    Ok(value) => {
      if let Value::Bool(bool_value) = value {
        if bool_value {
          Ok(())
        } else {
          let field_number = if field_context.field_data.tag == 0 {
            None
          } else {
            Some(field_context.field_data.tag as i32)
          };
          let mut elements = field_context.parent_elements.to_vec();
          let current_elem = FieldPathElement {
            field_type: Some(field_context.field_data.proto_type as i32),
            field_name: Some(field_context.field_data.proto_name.clone()),
            key_type: field_context.field_data.key_type.map(|t| t as i32),
            value_type: field_context.field_data.value_type.map(|t| t as i32),
            field_number,
            subscript: field_context.subscript,
          };
          elements.push(current_elem);
          Err(Violation {
            message: Some(message),
            rule_id: Some(rule_id),
            rule: None,
            field: Some(FieldPath { elements }),
            for_key: Some(field_context.field_data.is_for_key),
          })
        }
      } else {
        Err(Violation {
          message: Some(format!(
            "error during validation: expected boolean result, got `{:?}`",
            value.type_of()
          )),
          rule_id: None,
          rule: None,
          field: None,
          for_key: Some(false),
        })
      }
    }
    Err(e) => Err(Violation {
      message: Some(format!("error during validation: {:?}", e)),
      rule_id: None,
      rule: None,
      field: None,
      for_key: Some(false),
    }),
  }
}
