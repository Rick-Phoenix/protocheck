use cel_interpreter::{Context, Program, Value};

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::common::get_base_violations_path,
  ProtoType,
};

#[derive(Debug, Clone)]
pub struct CelRuleData<'a> {
  pub rule_id: String,
  pub error_message: String,
  pub rule_target: CelRuleTarget<'a>,
}

#[derive(Debug, Clone)]
pub enum CelRuleTarget<'a> {
  Message {
    name: String,
    parent_elements: &'a [FieldPathElement],
  },
  Field {
    field_context: FieldContext<'a>,
  },
}

impl CelRuleData<'_> {
  pub fn validation_type(&self) -> &str {
    match &self.rule_target {
      CelRuleTarget::Field { .. } => "field",
      CelRuleTarget::Message { .. } => "message",
    }
  }

  pub fn target_name(&self) -> &str {
    match &self.rule_target {
      CelRuleTarget::Field { field_context, .. } => &field_context.field_data.proto_name,
      CelRuleTarget::Message { name, .. } => name,
    }
  }

  pub fn is_for_message(&self) -> bool {
    matches!(&self.rule_target, CelRuleTarget::Message { .. })
  }
}

pub fn validate_cel<T>(
  rule_data: &CelRuleData,
  program: &'static Program,
  value: Option<&T>,
) -> Result<(), Violation>
where
  T: serde::Serialize,
{
  if value.is_none() {
    return Ok(());
  }

  let unwrapped_val = value.unwrap();
  let mut cel_context = Context::default();
  cel_context.add_variable("this", unwrapped_val).unwrap();

  let result = program.execute(&cel_context);

  let CelRuleData {
    rule_id,
    error_message,
    rule_target,
  } = rule_data;

  let validator_type = rule_data.validation_type();
  let target_name = rule_data.target_name();

  let error_prefix = format!(
    "Error during Cel validation for {} {}:",
    validator_type, target_name
  );

  match result {
    Ok(value) => {
      if let Value::Bool(bool_value) = value {
        if bool_value {
          Ok(())
        } else {
          let cel_violation = FieldPathElement {
            field_name: Some("cel".to_string()),
            field_number: Some(23),
            field_type: Some(ProtoType::Message as i32),
            key_type: None,
            value_type: None,
            subscript: None,
          };

          match rule_target {
            CelRuleTarget::Message {
              parent_elements, ..
            } => Err(Violation {
              message: Some(error_message.to_string()),
              rule_id: Some(rule_id.clone()),
              rule: Some(FieldPath {
                elements: vec![cel_violation.clone()],
              }),
              field: Some(FieldPath {
                elements: parent_elements.to_vec(),
              }),
              for_key: None,
            }),

            CelRuleTarget::Field { field_context } => {
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

              Err(Violation {
                message: Some(error_message.to_string()),
                rule_id: Some(rule_id.clone()),
                rule: Some(FieldPath {
                  elements: violations_path,
                }),
                field: Some(FieldPath { elements }),
                for_key: Some(field_context.field_data.kind.is_map_key()),
              })
            }
          }
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
