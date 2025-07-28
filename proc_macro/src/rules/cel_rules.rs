use cel_interpreter::Program;
use syn::Error;

use super::{FieldData, Rule, ValidatorCallTemplate, ValidatorKind};
use crate::Span2;

pub fn get_cel_rules(
  field_data: &FieldData,
  rules: &[Rule],
  is_for_message: bool,
) -> Result<Vec<ValidatorCallTemplate>, Error> {
  let mut validators: Vec<ValidatorCallTemplate> = Vec::new();

  for rule in rules {
    let program = Program::compile(rule.expression());

    match program {
      Ok(_) => {
        let expression = rule.expression().to_string();
        let message = rule.message().to_string();
        let rule_id = rule.id().to_string();
        let kind = if is_for_message {
          ValidatorKind::CelRule {
            expression,
            message,
            rule_id,
            is_for_message: true,
          }
        } else {
          ValidatorKind::CelRule {
            expression,
            message,
            rule_id,
            is_for_message: false,
          }
        };
        validators.push(ValidatorCallTemplate {
          field_data: field_data.clone(),
          kind,
        });
      }
      Err(e) => {
        return Err(syn::Error::new(
          Span2::call_site(),
          format!("Cel program failed to compile: {}", e),
        ))
      }
    }
  }

  Ok(validators)
}
