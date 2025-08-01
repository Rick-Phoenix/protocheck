use std::{collections::HashMap, sync::Arc};

use cel_interpreter::{objects::Key as CelKey, Context, Program, Value as CelValue};
use chrono::{DateTime, Utc};
use proc_macro2::TokenStream;
use prost_reflect::{DynamicMessage, FieldDescriptor, ReflectMessage, Value as ProstValue};
use quote::quote;
use syn::Error;

use super::{
  CelRuleTemplateTarget, FieldData, FieldValidator, MessageValidator, Rule, ValidatorKind,
  ValidatorTemplate,
};
use crate::{Ident2, Span2};

pub fn get_cel_rules(
  rule_target: &CelRuleTemplateTarget,
  rules: &[Rule],
  static_defs: &mut Vec<TokenStream>,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut validators: Vec<ValidatorTemplate> = Vec::new();

  let cel_value: CelValue = match rule_target {
    CelRuleTemplateTarget::Message(message_desc) => {
      let dyn_message = DynamicMessage::new(message_desc.clone());
      convert_prost_value_to_cel_value(&ProstValue::Message(dyn_message)).unwrap()
    }
    CelRuleTemplateTarget::Field(field_desc, validation_data) => {
      get_default_field_prost_value(&validation_data.field_data, field_desc).unwrap()
    }
  };

  let validation_type = rule_target.get_validation_type();
  let target_name = rule_target.get_full_name();

  let error_prefix = format!("Cel program error for {} {}:", validation_type, target_name);

  for rule in rules {
    let program = match Program::compile(rule.expression()) {
      Ok(prog) => prog,
      Err(e) => {
        return Err(syn::Error::new(
          Span2::call_site(),
          format!("{} failed to compile: {}", error_prefix, e),
        ))
      }
    };

    let mut context = Context::default();

    context.add_variable_from_value("this", &cel_value);

    match program.execute(&context) {
      Ok(result) => {
        if let CelValue::Bool(_) = result {
          let expression = rule.expression().to_string();
          let error_message = rule.message().to_string();
          let rule_id = rule.id().to_string();

          let static_program_ident = Ident2::new(
            &format!(
              "__CEL_{}_{}_PROGRAM",
              validation_type.to_uppercase(),
              target_name.to_uppercase(),
            ),
            Span2::call_site(),
          );

          let compilation_error = format!(
            "Cel program failed to compile for {} {}",
            validation_type, target_name,
          );

          static_defs.push(quote! {
            static #static_program_ident: std::sync::LazyLock<cel_interpreter::Program> = std::sync::LazyLock::new(|| {
              cel_interpreter::Program::compile(#expression).expect(#compilation_error)
            });
          });

          match rule_target {
            CelRuleTemplateTarget::Field(field_desc, validation_data) => {
              validators.push(ValidatorTemplate {
                item_rust_name: rule_target.get_name().to_string(),
                kind: ValidatorKind::Field {
                  validation_data: validation_data.clone(),
                  field_validator: FieldValidator::Cel {
                    full_name: validation_data.full_name.clone(),
                    message_full_name: field_desc.parent_message().full_name().to_string(),
                    rule_id,
                    error_message,
                    static_program_ident,
                  },
                },
              });
            }
            CelRuleTemplateTarget::Message(message_desc) => {
              validators.push(ValidatorTemplate {
                item_rust_name: rule_target.get_name().to_string(),
                kind: ValidatorKind::Message {
                  message_validator: MessageValidator::Cel {
                    full_name: message_desc.full_name().to_string(),
                    rule_id,
                    error_message,
                    static_program_ident,
                  },
                },
              });
            }
          };
        } else {
          return Err(Error::new(
            Span2::call_site(),
            format!(
              "{} expected boolean, got {}",
              error_prefix,
              result.type_of()
            ),
          ));
        }
      }
      Err(e) => {
        return Err(syn::Error::new(
          Span2::call_site(),
          format!("{} failed execution: {}", error_prefix, e),
        ))
      }
    };
  }

  Ok(validators)
}

fn get_default_field_prost_value(
  field_data: &FieldData,
  field_desc: &FieldDescriptor,
) -> Result<CelValue, Error> {
  let default_val = if field_data.kind.is_repeated_item() {
    ProstValue::default_value(&field_desc.kind())
  } else {
    ProstValue::default_value_for_field(field_desc)
  };

  convert_prost_value_to_cel_value(&default_val)
}

fn convert_prost_value_to_cel_value(prost_value: &ProstValue) -> Result<CelValue, Error> {
  match prost_value {
    ProstValue::F64(v) => Ok(CelValue::Float(*v)),
    ProstValue::F32(v) => Ok(CelValue::Float(*v as f64)),
    ProstValue::I32(v) => Ok(CelValue::Int(*v as i64)),
    ProstValue::I64(v) => Ok(CelValue::Int(*v)),
    ProstValue::U32(v) => Ok(CelValue::UInt(*v as u64)),
    ProstValue::U64(v) => Ok(CelValue::UInt(*v)),
    ProstValue::Bool(v) => Ok(CelValue::Bool(*v)),
    ProstValue::String(v) => Ok(CelValue::String(Arc::new(v.to_string()))),
    ProstValue::Bytes(v) => Ok(CelValue::Bytes(Arc::new(v.to_vec()))),
    ProstValue::EnumNumber(v) => Ok(CelValue::Int(*v as i64)),
    ProstValue::List(list_values) => {
      let cel_elements: Result<Vec<CelValue>, Error> = list_values
        .iter()
        .map(convert_prost_value_to_cel_value)
        .collect();
      Ok(CelValue::List(Arc::new(cel_elements?)))
    }
    ProstValue::Map(map_values) => {
      let mut cel_map = HashMap::new();
      for (key, val) in map_values.iter() {
        let cel_key = match key {
          prost_reflect::MapKey::String(s) => CelKey::String(Arc::new(s.clone())),
          prost_reflect::MapKey::I32(v) => CelKey::Int(*v as i64),
          prost_reflect::MapKey::I64(v) => CelKey::Int(*v),
          prost_reflect::MapKey::U32(v) => CelKey::Uint(*v as u64),
          prost_reflect::MapKey::U64(v) => CelKey::Uint(*v),
          prost_reflect::MapKey::Bool(v) => CelKey::Bool(*v),
        };
        let cel_val = convert_prost_value_to_cel_value(val)?;
        cel_map.insert(cel_key, cel_val);
      }
      Ok(CelValue::Map(cel_map.into()))
    }
    ProstValue::Message(dynamic_msg) => {
      let msg_desc = dynamic_msg.descriptor();
      let full_name = msg_desc.full_name();

      if full_name == "google.protobuf.Timestamp" {
        let utc: DateTime<Utc> = Utc::now();
        Ok(CelValue::Timestamp(utc.fixed_offset()))
      } else if full_name == "google.protobuf.Duration" {
        Ok(CelValue::Duration(chrono::Duration::default()))
      } else {
        let mut cel_map = HashMap::new();
        for field in msg_desc.fields() {
          if field.containing_oneof().is_some() {
            continue;
          }
          let cel_field_name = CelKey::String(Arc::new(field.name().to_string()));
          let cel_field_value =
            convert_prost_value_to_cel_value(&ProstValue::default_value(&field.kind()))?;
          cel_map.insert(cel_field_name, cel_field_value);
        }
        Ok(CelValue::Map(cel_map.into()))
      }
    }
  }
}
