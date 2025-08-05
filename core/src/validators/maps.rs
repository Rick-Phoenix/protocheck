use std::collections::HashMap;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, FieldPathElement, Violation},
  ProtoType,
};

pub fn min_pairs<K, V>(
  field_context: &FieldContext,
  value: &HashMap<K, V>,
  min_pairs: u64,
) -> Result<(), Violation> {
  let check = value.len() >= min_pairs as usize;

  if !check {
    let plural_suffix = if min_pairs > 1 { "s" } else { "" };

    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(ProtoType::Message as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.key_type.map(|t| t as i32),
      value_type: field_context.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript.clone(),
    };
    elements.push(current_elem);
    let violation = Violation {
      rule_id: Some("map.min_pairs".to_string()),
      message: Some(format!(
        "map field `{}` requires at least {} item{}",
        field_context.field_data.proto_name.clone(),
        min_pairs,
        plural_suffix
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: vec![
          FieldPathElement {
            field_name: Some("map".to_string()),
            field_number: Some(19),
            field_type: Some(ProtoType::Message as i32),
            subscript: None,
            key_type: None,
            value_type: None,
          },
          FieldPathElement {
            field_name: Some("min_pairs".to_string()),
            field_number: Some(1),
            field_type: Some(ProtoType::Uint64 as i32),
            key_type: None,
            value_type: None,
            subscript: None,
          },
        ],
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn max_pairs<K, V>(
  field_context: &FieldContext,
  value: &HashMap<K, V>,
  max_pairs: u64,
) -> Result<(), Violation> {
  let check = value.len() <= max_pairs as usize;

  if !check {
    let plural_suffix = if max_pairs > 1 { "s" } else { "" };

    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(ProtoType::Message as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.key_type.map(|t| t as i32),
      value_type: field_context.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript.clone(),
    };
    elements.push(current_elem);
    let violation = Violation {
      rule_id: Some("map.max_pairs".to_string()),
      message: Some(format!(
        "map field `{}` cannot have more than {} item{}",
        field_context.field_data.proto_name.clone(),
        max_pairs,
        plural_suffix
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: vec![
          FieldPathElement {
            field_name: Some("map".to_string()),
            field_number: Some(19),
            field_type: Some(ProtoType::Message as i32),
            subscript: None,
            key_type: None,
            value_type: None,
          },
          FieldPathElement {
            field_name: Some("max_pairs".to_string()),
            field_number: Some(2),
            field_type: Some(ProtoType::Uint64 as i32),
            key_type: None,
            value_type: None,
            subscript: None,
          },
        ],
      }),
    };
    return Err(violation);
  };
  Ok(())
}
