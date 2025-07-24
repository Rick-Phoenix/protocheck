use crate::{
  field_data::{FieldContext, FieldData},
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::common::get_base_violations_path,
  ProtoTypes,
};

pub fn max_len(
  field_context: FieldContext,
  value: Option<&str>,
  max_len: usize,
) -> Result<(), Violation> {
  let check = if value.is_some() {
    let unwrapped_val = value.unwrap();
    unwrapped_val.chars().count() <= max_len
  } else {
    return Ok(());
  };

  let plural_suffix = if max_len > 1 {
    format!("s")
  } else {
    format!("")
  };

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(ProtoTypes::String.into()),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.field_data.key_type.map(|t| t as i32),
      value_type: field_context.field_data.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript,
    };

    elements.push(current_elem);

    let FieldData {
      is_repeated_item,
      is_map_key,
      is_map_value,
      ..
    } = field_context.field_data;

    let mut violation_elements =
      get_base_violations_path(is_repeated_item, is_map_key, is_map_value);

    violation_elements.extend(vec![
      FieldPathElement {
        field_name: Some("string".to_string()),
        field_number: Some(14),
        field_type: Some(ProtoTypes::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("max_len".to_string()),
        field_number: Some(3),
        field_type: Some(ProtoTypes::Uint64 as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]);

    let violation = Violation {
      rule_id: Some("string.max_len".to_string()),
      message: Some(format!(
        "`{}` cannot be longer than {} character{}",
        field_context.field_data.proto_name.clone(),
        max_len,
        plural_suffix
      )),
      for_key: Some(field_context.field_data.is_map_key),
      field: Some(FieldPath { elements: elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn min_len(
  field_context: FieldContext,
  value: Option<&str>,
  min_len: usize,
) -> Result<(), Violation> {
  let check = if value.is_some() {
    let unwrapped_val = value.unwrap();
    unwrapped_val.chars().count() >= min_len
  } else {
    return Ok(());
  };

  let plural_suffix = if min_len > 1 {
    format!("s")
  } else {
    format!("")
  };

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(ProtoTypes::String.into()),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.field_data.key_type.map(|t| t as i32),
      value_type: field_context.field_data.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript,
    };

    elements.push(current_elem);

    let FieldData {
      is_repeated_item,
      is_map_key,
      is_map_value,
      ..
    } = field_context.field_data;

    let mut violation_elements =
      get_base_violations_path(is_repeated_item, is_map_key, is_map_value);

    violation_elements.extend(vec![
      FieldPathElement {
        field_name: Some("string".to_string()),
        field_number: Some(14),
        field_type: Some(ProtoTypes::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("min_len".to_string()),
        field_number: Some(2),
        field_type: Some(ProtoTypes::Uint64 as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]);

    let violation = Violation {
      rule_id: Some("string.min_len".to_string()),
      message: Some(format!(
        "`{}` cannot be shorter than {} character{}",
        field_context.field_data.proto_name.clone(),
        min_len,
        plural_suffix
      )),
      for_key: Some(field_context.field_data.is_map_key),
      field: Some(FieldPath { elements: elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn len(field_context: FieldContext, value: Option<&str>, len: usize) -> Result<(), Violation> {
  let check = if value.is_some() {
    let unwrapped_val = value.unwrap();
    unwrapped_val.chars().count() == len
  } else {
    return Ok(());
  };

  let plural_suffix = if len > 1 { format!("s") } else { format!("") };

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(ProtoTypes::String.into()),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.field_data.key_type.map(|t| t as i32),
      value_type: field_context.field_data.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript,
    };

    elements.push(current_elem);

    let FieldData {
      is_repeated_item,
      is_map_key,
      is_map_value,
      ..
    } = field_context.field_data;

    let mut violation_elements =
      get_base_violations_path(is_repeated_item, is_map_key, is_map_value);

    violation_elements.extend(vec![
      FieldPathElement {
        field_name: Some("string".to_string()),
        field_number: Some(14),
        field_type: Some(ProtoTypes::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("len".to_string()),
        field_number: Some(19),
        field_type: Some(ProtoTypes::Uint64 as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]);

    let violation = Violation {
      rule_id: Some("string.len".to_string()),
      message: Some(format!(
        "`{}` must be exactly {} character{} long",
        field_context.field_data.proto_name.clone(),
        len,
        plural_suffix
      )),
      for_key: Some(field_context.field_data.is_map_key),
      field: Some(FieldPath { elements: elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}
