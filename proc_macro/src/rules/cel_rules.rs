#[cfg(not(feature = "cel"))]
#[deprecated(
  since = "0.1.0",
  note = "Cannot use CEL validators without enabling the 'cel' feature in your Cargo.toml."
)]
pub fn get_cel_rules(
  _: &crate::cel_rule_template::CelRuleTemplateTarget,
  _: &[Rule],
  _: &mut Vec<proc_macro2::TokenStream>,
) -> Result<proc_macro2::TokenStream, syn::Error> {
  unimplemented!("Cannot use Cel validators without the 'cel' feature")
}

#[cfg(feature = "cel")]
mod cel {
  use std::{collections::HashMap, sync::Arc};

  use cel::{objects::Key as CelKey, Context, Program, Value as CelValue};
  use proc_macro2::TokenStream;
  use prost_reflect::{DynamicMessage, FieldDescriptor, ReflectMessage, Value as ProstValue};
  use proto_types::{Duration, Empty, FieldMask, FieldType, Timestamp};
  use quote::quote;
  use syn::Error;

  use super::super::Rule;
  use crate::{
    cel_rule_template::CelRuleTemplateTarget, validation_data::ValidationData, Ident2, Span2,
  };

  pub fn get_cel_rules(
    rule_target: &CelRuleTemplateTarget,
    rules: &[Rule],
    static_defs: &mut Vec<TokenStream>,
  ) -> Result<TokenStream, Error> {
    let mut tokens = TokenStream::new();

    let cel_value: CelValue = match rule_target {
      CelRuleTemplateTarget::Message { message_desc, .. } => {
        let dyn_message = DynamicMessage::new((*message_desc).clone());
        convert_prost_value_to_cel_value(&ProstValue::Message(dyn_message)).unwrap()
      }
      CelRuleTemplateTarget::Field {
        field_desc,
        validation_data,
        ..
      } => get_default_field_prost_value(validation_data, field_desc).unwrap(),
    };

    let validation_type = rule_target.get_validation_type();
    let target_name = rule_target.get_full_name();

    let error_prefix = format!("Cel program error for {} {}:", validation_type, target_name);

    let (parent_messages_ident, violations_ident) = rule_target.get_idents();

    for (index, rule) in rules.iter().enumerate() {
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
              &format!("__CEL_PROGRAM_{}_{}", target_name.to_uppercase(), index),
              Span2::call_site(),
            );

            let compilation_error = format!(
              "Cel program failed to compile for {} {}",
              validation_type, target_name,
            );

            static_defs.push(quote! {
            static #static_program_ident: std::sync::LazyLock<::cel::Program> = std::sync::LazyLock::new(|| {
              ::cel::Program::compile(#expression).expect(#compilation_error)
            });
          });

            let rule_tokens = quote! {
              ::protocheck::validators::cel::CelRule {
                id: #rule_id,
                error_message: #error_message,
                program: &#static_program_ident,
                item_full_name: #target_name
              }
            };

            match rule_target {
              CelRuleTemplateTarget::Field {
                is_boxed,
                validation_data,
                ..
              } => {
                let field_context_ident = &validation_data.field_context_ident();
                let value_ident = &validation_data.value_ident();

                let value_tokens = if *is_boxed {
                  quote! { *val }
                } else {
                  quote! { #value_ident }
                };

                let validation_expression = match validation_data.field_kind.inner_type() {
                  FieldType::Message => {
                    quote! { validate_cel_field_try_into(&#field_context_ident, &rule, #value_tokens.clone()) }
                  }
                  FieldType::Timestamp | FieldType::Duration => {
                    quote! { validate_cel_field_try_into(&#field_context_ident, &rule, #value_tokens) }
                  }
                  FieldType::Float => {
                    quote! { validate_cel_field_with_val(&#field_context_ident, &rule, (#value_tokens as f64).into()) }
                  }
                  FieldType::Bytes => {
                    quote! { validate_cel_field_with_val(&#field_context_ident, &rule, #value_tokens.to_vec().into()) }
                  }
                  FieldType::Int32 | FieldType::Sint32 | FieldType::Sfixed32 => {
                    quote! { validate_cel_field_with_val(&#field_context_ident, &rule, (#value_tokens as i64).into()) }
                  }
                  FieldType::Uint32 | FieldType::Fixed32 => {
                    quote! { validate_cel_field_with_val(&#field_context_ident, &rule, (#value_tokens as u64).into()) }
                  }
                  _ => {
                    quote! { validate_cel_field_with_val(&#field_context_ident, &rule, (#value_tokens).clone().into()) }
                  }
                };

                let validator_tokens = quote! {
                  let rule = #rule_tokens;

                  match ::protocheck::validators::cel::#validation_expression {
                    Ok(_) => {}
                    Err(v) => #violations_ident.push(v)
                  };
                };

                tokens.extend(validator_tokens);
              }
              CelRuleTemplateTarget::Message { .. } => {
                let validator_tokens = quote! {
                  let rule = #rule_tokens;

                  match ::protocheck::validators::cel::validate_cel_message(#parent_messages_ident, &rule, self.clone()) {
                    Ok(_) => {}
                    Err(v) => #violations_ident.push(v)
                  };
                };

                tokens.extend(validator_tokens);
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

    Ok(tokens)
  }

  fn get_default_field_prost_value(
    validation_data: &ValidationData,
    field_desc: &FieldDescriptor,
  ) -> Result<CelValue, Error> {
    let default_val = if validation_data.field_kind.is_repeated_item() {
      ProstValue::default_value(&field_desc.kind())
    } else {
      ProstValue::default_value_for_field(field_desc)
    };

    convert_prost_value_to_cel_value(&default_val)
  }

  fn convert_prost_value_to_cel_value(prost_value: &ProstValue) -> Result<CelValue, Error> {
    convert_prost_value_to_cel_value_recursive(prost_value, 0)
  }

  const MAX_RECURSION_DEPTH: usize = 10;

  fn convert_prost_value_to_cel_value_recursive(
    prost_value: &ProstValue,
    depth: usize,
  ) -> Result<CelValue, Error> {
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
          let cel_val = convert_prost_value_to_cel_value_recursive(val, depth + 1)?;
          cel_map.insert(cel_key, cel_val);
        }
        Ok(CelValue::Map(cel_map.into()))
      }
      ProstValue::Message(dynamic_msg) => {
        let msg_desc = dynamic_msg.descriptor();
        let full_name = msg_desc.full_name();

        match full_name {
          "google.protobuf.Timestamp" => Ok(CelValue::Timestamp(
            Timestamp::default().try_into().unwrap(),
          )),
          "google.protobuf.Empty" => Ok(Empty {}.into()),
          "google.protobuf.FieldMask" => Ok(FieldMask::new(vec![]).into()),
          "google.protobuf.Duration" => {
            Ok(CelValue::Duration(Duration::default().try_into().unwrap()))
          }
          _ => {
            if depth >= MAX_RECURSION_DEPTH {
              return Ok(CelValue::Map(HashMap::<CelKey, CelValue>::new().into()));
            }
            let mut cel_map = HashMap::new();
            for field in msg_desc.fields() {
              if field.containing_oneof().is_some() {
                continue;
              }
              let cel_field_name = CelKey::String(Arc::new(field.name().to_string()));
              let cel_field_value = convert_prost_value_to_cel_value_recursive(
                &ProstValue::default_value(&field.kind()),
                depth + 1,
              )?;
              cel_map.insert(cel_field_name, cel_field_value);
            }
            Ok(CelValue::Map(cel_map.into()))
          }
        }
      }
    }
  }
}

#[cfg(feature = "cel")]
pub use cel::*;
