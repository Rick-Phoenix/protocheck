use proto_types::protovalidate::Ignore;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::static_data::base_violations::get_base_violations_path,
  ProtoType,
};

pub fn max_len(field_context: &FieldContext, value: &str, max_len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.chars().count() <= max_len as usize;

  let plural_suffix = if max_len > 1 { "s" } else { "" };

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(ProtoType::String as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.field_data.key_type.map(|t| t as i32),
      value_type: field_context.field_data.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript.clone(),
    };

    elements.push(current_elem);

    let mut violation_elements = get_base_violations_path(&field_context.field_data.kind);

    violation_elements.extend(vec![
      FieldPathElement {
        field_name: Some("string".to_string()),
        field_number: Some(14),
        field_type: Some(ProtoType::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("max_len".to_string()),
        field_number: Some(3),
        field_type: Some(ProtoType::Uint64 as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]);

    let violation = Violation {
      rule_id: Some("string.max_len".to_string()),
      message: Some(format!(
        "{} cannot be longer than {} character{}",
        field_context.field_data.proto_name.clone(),
        max_len,
        plural_suffix
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn min_len(field_context: &FieldContext, value: &str, min_len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.chars().count() >= min_len as usize;

  let plural_suffix = if min_len > 1 { "s" } else { "" };

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(ProtoType::String as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.field_data.key_type.map(|t| t as i32),
      value_type: field_context.field_data.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript.clone(),
    };

    elements.push(current_elem);

    let mut violation_elements = get_base_violations_path(&field_context.field_data.kind);

    violation_elements.extend(vec![
      FieldPathElement {
        field_name: Some("string".to_string()),
        field_number: Some(14),
        field_type: Some(ProtoType::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("min_len".to_string()),
        field_number: Some(2),
        field_type: Some(ProtoType::Uint64 as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]);

    let violation = Violation {
      rule_id: Some("string.min_len".to_string()),
      message: Some(format!(
        "{} cannot be shorter than {} character{}",
        field_context.field_data.proto_name.clone(),
        min_len,
        plural_suffix
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn len(field_context: &FieldContext, value: &str, len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.chars().count() == len as usize;

  let plural_suffix = if len > 1 { "s" } else { "" };

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(ProtoType::String as i32),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.field_data.key_type.map(|t| t as i32),
      value_type: field_context.field_data.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript.clone(),
    };

    elements.push(current_elem);

    let mut violation_elements = get_base_violations_path(&field_context.field_data.kind);

    violation_elements.extend(vec![
      FieldPathElement {
        field_name: Some("string".to_string()),
        field_number: Some(14),
        field_type: Some(ProtoType::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("len".to_string()),
        field_number: Some(19),
        field_type: Some(ProtoType::Uint64 as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]);

    let violation = Violation {
      rule_id: Some("string.len".to_string()),
      message: Some(format!(
        "{} must be exactly {} character{} long",
        field_context.field_data.proto_name.clone(),
        len,
        plural_suffix
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}
