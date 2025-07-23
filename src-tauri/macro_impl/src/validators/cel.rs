use cel_interpreter::{Context, Program, Value};
use proto_types::buf::validate::Violation;

pub fn validate_cel(
  program: &'static Program,
  context: Context,
  message: String,
) -> Result<(), Violation> {
  let result = program.execute(&context);

  match result {
    Ok(value) => {
      if let Value::Bool(bool_value) = value {
        if bool_value {
          Ok(())
        } else {
          Err(Violation {
            message: Some(message),
            rule_id: None,
            rule: None,
            field: None,
            for_key: Some(false),
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
