use std::{collections::HashMap, sync::Arc};

use cel_interpreter::{objects::Key as CelKey, Context, Program, Value as CelValue};
use chrono::{DateTime, Utc};
use proc_macro2::TokenStream;
use prost_reflect::{DynamicMessage, FieldDescriptor, ReflectMessage, Value as ProstValue};
use protocheck_core::field_data::CelRuleTarget;
use quote::quote;
use random_string::charsets::ALPHA_LOWER;
use syn::Error;

use super::{FieldData, Rule, ValidatorKind, ValidatorTemplate};
use crate::{Ident2, Span2};

pub fn get_cel_rules(
  rule_kind: &CelRuleTarget,
  rules: &[Rule],
  static_defs: &mut Vec<TokenStream>,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut validators: Vec<ValidatorTemplate> = Vec::new();

  let cel_value: CelValue = match rule_kind {
    CelRuleTarget::Message(message_desc) => {
      let dyn_message = DynamicMessage::new(message_desc.clone());
      convert_prost_value_to_cel_value(&ProstValue::Message(dyn_message)).unwrap()
    }
    CelRuleTarget::Field(field_desc, field_data) => {
      get_default_field_prost_value(field_data, field_desc).unwrap()
    }
  };

  let validation_type = rule_kind.get_validation_type();
  let target_name = rule_kind.get_name();

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
          let message = rule.message().to_string();
          let rule_id = rule.id().to_string();

          let random_string = random_string::generate(5, ALPHA_LOWER);
          let static_program_ident = Ident2::new(
            &format!(
              "__CEL_{}_{}_PROGRAM_{}",
              validation_type.to_uppercase(),
              rule_kind.get_name(),
              random_string
            ),
            Span2::call_site(),
          );

          let compilation_error = format!(
            "Cel program failed to compile for {} {}",
            validation_type,
            rule_kind.get_name()
          );

          static_defs.push(quote! {
              #[allow(non_upper_case_globals)]
              static #static_program_ident: std::sync::LazyLock<cel_interpreter::Program> = std::sync::LazyLock::new(|| {
                cel_interpreter::Program::compile(#expression).expect(#compilation_error)
              });
            });

          validators.push(ValidatorTemplate {
            item_rust_name: rule_kind.get_name().to_string(),
            kind: ValidatorKind::CelRule {
              error_message: message,
              rule_id,
              rule_template: rule_kind.clone(),
              static_program_ident,
            },
          });
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
