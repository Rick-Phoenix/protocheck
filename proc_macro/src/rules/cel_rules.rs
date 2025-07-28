use std::collections::HashMap;

use cel_interpreter::{Context, Program, Value as CelValue};
use prost_reflect::{
  DynamicMessage, FieldDescriptor, Kind, MapKey, MessageDescriptor, SerializeOptions,
  Value as ProstValue,
};
use serde_json::{Serializer, Value as JsonValue};
use syn::Error;

use super::{FieldData, Rule, ValidatorCallTemplate, ValidatorKind};
use crate::Span2;

pub fn get_cel_rules(
  message_desc: &MessageDescriptor,
  field_data: &FieldData,
  rules: &[Rule],
  is_for_message: bool,
) -> Result<Vec<ValidatorCallTemplate>, Error> {
  let mut validators: Vec<ValidatorCallTemplate> = Vec::new();

  for rule in rules {
    let program = match Program::compile(rule.expression()) {
      Ok(prog) => prog,
      Err(e) => {
        return Err(syn::Error::new(
          Span2::call_site(),
          format!("Cel program failed to compile: {}", e),
        ))
      }
    };

    let dyn_message = DynamicMessage::new(message_desc.clone());
    let serialize_options = SerializeOptions::new().skip_default_fields(false);
    let mut serializer = Serializer::new(vec![]);

    let json_val: serde_json::Value = if is_for_message {
      dyn_message
        .serialize_with_options(&mut serializer, &serialize_options)
        .unwrap();
      serde_json::from_slice(&serializer.into_inner()).unwrap()
    } else {
      let field = message_desc.get_field(field_data.tag).unwrap();

      let default_val = if field_data.is_repeated_item {
        ProstValue::default_value(&field.kind())
      } else {
        ProstValue::default_value_for_field(&field)
      };
      let field_json_val = convert_prost_value_to_json_value(&default_val).unwrap();

      serde_json::from_value(field_json_val).unwrap()
    };

    println!("Dyn Message: {:#?}", json_val);

    let mut context = Context::default();
    match context.add_variable("this", json_val) {
      Ok(_) => match program.execute(&context) {
        Ok(result) => {
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
            format!("Cel program failed to execute: {}", e),
          ))
        }
      },
      Err(e) => {
        return Err(syn::Error::new(
          Span2::call_site(),
          format!("Cel program failed to add context: {}", e),
        ))
      }
    };
  }

  Ok(validators)
}

fn convert_prost_value_to_json_value(prost_value: &ProstValue) -> Result<JsonValue, syn::Error> {
  match prost_value {
    ProstValue::F64(v) => Ok(JsonValue::from(*v)),
    ProstValue::F32(v) => Ok(JsonValue::from(*v)),
    ProstValue::I32(v) => Ok(JsonValue::from(*v)),
    ProstValue::I64(v) => Ok(JsonValue::from(*v)),
    ProstValue::U32(v) => Ok(JsonValue::from(*v)),
    ProstValue::U64(v) => Ok(JsonValue::from(*v)),
    ProstValue::Bool(v) => Ok(JsonValue::from(*v)),
    ProstValue::String(v) => Ok(JsonValue::from(v.as_str())),
    ProstValue::Bytes(v) => Ok(JsonValue::Array(
      v.iter().map(|&byte| JsonValue::from(byte)).collect(),
    )),
    ProstValue::EnumNumber(v) => Ok(JsonValue::from(*v)),
    ProstValue::List(list_values) => {
      let json_elements: Result<Vec<JsonValue>, syn::Error> = list_values
        .iter()
        .map(convert_prost_value_to_json_value)
        .collect();
      Ok(JsonValue::Array(json_elements?))
    }
    ProstValue::Map(map_values) => {
      let mut json_map = serde_json::Map::new();
      let serialize_options = SerializeOptions::new().skip_default_fields(false);
      for (key, val) in map_values.iter() {
        let json_key_as_string = match key {
          prost_reflect::MapKey::String(s) => s.clone(),
          prost_reflect::MapKey::I32(v) => v.to_string(),
          prost_reflect::MapKey::I64(v) => v.to_string(),
          prost_reflect::MapKey::U32(v) => v.to_string(),
          prost_reflect::MapKey::U64(v) => v.to_string(),
          prost_reflect::MapKey::Bool(v) => v.to_string(),
        };
        let json_val = if let ProstValue::Message(dynamic_msg) = val {
          let mut serializer = serde_json::Serializer::new(Vec::new());
          dynamic_msg
            .serialize_with_options(&mut serializer, &serialize_options)
            .map_err(|e| {
              syn::Error::new(
                Span2::call_site(),
                format!("Failed to serialize map value DynamicMessage: {}", e),
              )
            })?;
          serde_json::from_slice(&serializer.into_inner()).map_err(|e| {
            syn::Error::new(
              Span2::call_site(),
              format!(
                "Failed to parse serialized map value bytes to JsonValue: {}",
                e
              ),
            )
          })?
        } else {
          convert_prost_value_to_json_value(val)?
        };

        json_map.insert(json_key_as_string, json_val);
      }
      Ok(JsonValue::Object(json_map))
    }
    ProstValue::Message(dynamic_msg) => {
      let serialize_options = SerializeOptions::new().skip_default_fields(false);
      let mut serializer = Serializer::new(Vec::new());
      dynamic_msg
        .serialize_with_options(&mut serializer, &serialize_options)
        .map_err(|e| {
          syn::Error::new(
            Span2::call_site(),
            format!("Failed to serialize nested DynamicMessage: {}", e),
          )
        })?;
      Ok(
        serde_json::from_slice::<JsonValue>(&serializer.into_inner()).map_err(|e| {
          syn::Error::new(
            Span2::call_site(),
            format!(
              "Failed to parse serialized nested bytes to JsonValue: {}",
              e
            ),
          )
        })?,
      )
    }
  }
}
