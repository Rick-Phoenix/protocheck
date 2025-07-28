use cel_interpreter::{Context, Program, Value as CelValue};
use prost_reflect::{
  DynamicMessage, FieldDescriptor, MessageDescriptor, SerializeOptions, Value as ProstValue,
};
use serde_json::{Serializer, Value as JsonValue};
use syn::Error;

use super::{FieldData, Rule, ValidatorCallTemplate, ValidatorKind};
use crate::Span2;

#[derive(Debug, Clone)]
pub enum CelRuleKind<'a> {
  Message(&'a MessageDescriptor),
  Field(&'a FieldDescriptor),
}

pub fn get_cel_rules(
  rule_kind: &CelRuleKind,
  field_data: &FieldData,
  rules: &[Rule],
) -> Result<Vec<ValidatorCallTemplate>, Error> {
  let mut validators: Vec<ValidatorCallTemplate> = Vec::new();
  let mut is_for_message = false;

  let json_val: JsonValue = match rule_kind {
    CelRuleKind::Message(message_desc) => {
      is_for_message = true;
      let dyn_message = DynamicMessage::new(message_desc.to_owned().clone());
      convert_prost_value_to_json_value(&ProstValue::Message(dyn_message))?
    }
    CelRuleKind::Field(field_desc) => get_default_field_prost_value(field_data, field_desc)?,
  };

  let validator_type = match is_for_message {
    true => "message",
    false => "field",
  };
  let error_prefix = format!(
    "Cel program error for {} {}:",
    validator_type, field_data.proto_name
  );

  let serialized_json_val: JsonValue = serde_json::from_value(json_val).map_err(|e| {
    Error::new(
      Span2::call_site(),
      format!(
        "{} failed to serialize descriptor (ensure the message implements serde::Serialize): {}",
        error_prefix, e
      ),
    )
  })?;

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
    match context.add_variable("this", &serialized_json_val) {
      Ok(_) => match program.execute(&context) {
        Ok(result) => {
          if let CelValue::Bool(_) = result {
            let expression = rule.expression().to_string();
            let message = rule.message().to_string();
            let rule_id = rule.id().to_string();

            validators.push(ValidatorCallTemplate {
              field_data: field_data.clone(),
              kind: ValidatorKind::CelRule {
                expression,
                message,
                rule_id,
                is_for_message,
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
      },
      Err(e) => {
        return Err(syn::Error::new(
          Span2::call_site(),
          format!("{} failed to add context: {}", error_prefix, e),
        ))
      }
    };
  }

  Ok(validators)
}

fn get_default_field_prost_value(
  field_data: &FieldData,
  field_desc: &FieldDescriptor,
) -> Result<JsonValue, Error> {
  let default_val = if field_data.is_repeated_item {
    ProstValue::default_value(&field_desc.kind())
  } else {
    ProstValue::default_value_for_field(field_desc)
  };

  convert_prost_value_to_json_value(&default_val)
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
                format!("Failed to serialize map value: {}", e),
              )
            })?;
          serde_json::from_slice(&serializer.into_inner()).map_err(|e| {
            syn::Error::new(
              Span2::call_site(),
              format!("Failed to parse serialized map value bytes: {}", e),
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
            format!("Failed to serialize nested message: {}", e),
          )
        })?;
      Ok(
        serde_json::from_slice::<JsonValue>(&serializer.into_inner()).map_err(|e| {
          syn::Error::new(
            Span2::call_site(),
            format!("Failed to serialize nested message bytes: {}", e),
          )
        })?,
      )
    }
  }
}
