use proto_types::{protovalidate::Violation, Duration, Timestamp};

use crate::{
  field_data::FieldContext,
  protovalidate::FieldPath,
  validators::{static_data::base_violations::get_base_violations_path, FieldPathElement},
  ProtoType,
};

pub fn within(
  field_context: &FieldContext,
  value: &Timestamp,
  time_range: &Duration,
) -> Result<(), Violation> {
  let check = value.is_within_range_from_now(time_range);

  if !check {
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

    let mut violation_elements = get_base_violations_path(&field_context.field_data.kind);

    violation_elements.extend(vec![
      FieldPathElement {
        field_name: Some("timestamp".to_string()),
        field_number: Some(22),
        field_type: Some(ProtoType::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("within".to_string()),
        field_number: Some(9),
        field_type: Some(ProtoType::Message as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]);

    let violation = Violation {
      rule_id: Some("timestamp.within".to_string()),
      message: Some(format!(
        "{} has to be within {} from now",
        field_context.field_data.proto_name.clone(),
        time_range.display_full(),
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn lt_now(field_context: &FieldContext, value: Timestamp, _: ()) -> Result<(), Violation> {
  let check = value.is_past();

  if !check {
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

    let mut violation_elements = get_base_violations_path(&field_context.field_data.kind);

    violation_elements.extend(vec![
      FieldPathElement {
        field_name: Some("timestamp".to_string()),
        field_number: Some(22),
        field_type: Some(ProtoType::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("lt_now".to_string()),
        field_number: Some(7),
        field_type: Some(ProtoType::Bool as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]);

    let violation = Violation {
      rule_id: Some("timestamp.lt_now".to_string()),
      message: Some(format!(
        "{} has to be in the past",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn gt_now(field_context: &FieldContext, value: Timestamp, _: ()) -> Result<(), Violation> {
  let check = value.is_future();

  if !check {
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

    let mut violation_elements = get_base_violations_path(&field_context.field_data.kind);

    violation_elements.extend(vec![
      FieldPathElement {
        field_name: Some("timestamp".to_string()),
        field_number: Some(22),
        field_type: Some(ProtoType::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("lt_now".to_string()),
        field_number: Some(8),
        field_type: Some(ProtoType::Bool as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]);

    let violation = Violation {
      rule_id: Some("timestamp.gt_now".to_string()),
      message: Some(format!(
        "{} has to be in the future",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}
