use cel_interpreter::Program;
use proc_macro2::Span;
use proto_types::{buf::validate::Rule, FieldData, GeneratedCodeKind, ValidatorCallTemplate};

pub fn get_cel_rules(rules: Vec<Rule>) -> Result<Vec<ValidatorCallTemplate>, syn::Error> {
  let mut validators: Vec<ValidatorCallTemplate> = Vec::new();

  for rule in rules {
    let program = Program::compile(rule.expression());

    match program {
      Ok(_) => {
        let mut field_data = FieldData::default();
        field_data.rust_name = "test".to_string();
        field_data.proto_name = "test".to_string();

        validators.push(ValidatorCallTemplate {
          field_data,
          validator_path: None,
          target_value_tokens: None,
          kind: GeneratedCodeKind::CelRule {
            expression: rule.expression().to_string(),
            message: rule.message().to_string(),
          },
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
