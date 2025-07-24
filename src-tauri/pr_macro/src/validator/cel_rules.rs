use cel_interpreter::Program;
use proc_macro2::Span;
use proto_types::{buf::validate::Rule, FieldData, GeneratedCodeKind, ValidatorCallTemplate};

pub fn get_cel_rules(
  field_data: &FieldData,
  rules: Vec<Rule>,
  is_for_message: bool,
) -> Result<Vec<ValidatorCallTemplate>, syn::Error> {
  let mut validators: Vec<ValidatorCallTemplate> = Vec::new();

  for rule in rules {
    let program = Program::compile(rule.expression());

    match program {
      Ok(_) => {
        let expression = rule.expression().to_string();
        let message = rule.message().to_string();
        let rule_id = rule.id().to_string();
        let kind = if is_for_message {
          GeneratedCodeKind::MessageCelRule {
            expression,
            message,
            rule_id,
          }
        } else {
          GeneratedCodeKind::FieldCelRule {
            expression,
            message,
            rule_id,
          }
        };
        validators.push(ValidatorCallTemplate {
          field_data: field_data.clone(),
          validator_path: None,
          target_value_tokens: None,
          kind,
        });
      }
      Err(e) => {
        return Err(syn::Error::new(
          Span::call_site(),
          format!("Cel program failed to compile: {}", e.to_string()),
        ))
      }
    }
  }

  Ok(validators)
}
