use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::static_data::base_violations::get_base_violations_path,
  ProtoType,
};

pub fn f32_is_finite(
  field_context: &FieldContext,
  value: Option<f32>,
  target: bool,
) -> Result<(), Violation> {
  let check = match value {
    Some(val) => val.is_nan() == target,
    None => return Ok(()),
  };

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(ProtoType::Float as i32),
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
        field_name: Some("float".to_string()),
        field_number: Some(1),
        field_type: Some(ProtoType::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("finite".to_string()),
        field_number: Some(8),
        field_type: Some(ProtoType::Bool as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]);

    let violation = Violation {
      rule_id: Some("float.finite".to_string()),
      message: Some(format!(
        "{} must be a finite number",
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

pub fn f64_is_finite(
  field_context: &FieldContext,
  value: Option<f64>,
  target: bool,
) -> Result<(), Violation> {
  let check = match value {
    Some(val) => val.is_nan() == target,
    None => return Ok(()),
  };

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(ProtoType::Double as i32),
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
        field_name: Some("double".to_string()),
        field_number: Some(2),
        field_type: Some(ProtoType::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("finite".to_string()),
        field_number: Some(8),
        field_type: Some(ProtoType::Bool as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]);

    let violation = Violation {
      rule_id: Some("double.finite".to_string()),
      message: Some(format!(
        "{} must be a finite number",
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
