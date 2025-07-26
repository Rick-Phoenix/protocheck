use cel_interpreter::Program;
use syn::{Error, Ident};

use super::{FieldData, GeneratedCodeKind, Rule, ValidatorCallTemplate};
use crate::Span2;

pub fn get_cel_rules(
  oneof_ident: Option<Ident>,
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
          GeneratedCodeKind::CelRule {
            expression,
            message,
            rule_id,
            is_for_message: true,
          }
        } else {
          GeneratedCodeKind::CelRule {
            expression,
            message,
            rule_id,
            is_for_message: false,
          }
        };
        validators.push(ValidatorCallTemplate {
          field_data: field_data.clone(),
          validator_path: None,
          target_value_tokens: None,
          kind,
          oneof_ident: oneof_ident.clone(),
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
