use super::*;

pub static ENUM_DEFINED_ONLY_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("enum".to_string()),
      field_number: Some(16),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("defined_only".to_string()),
      field_number: Some(2),
      field_type: Some(Type::Bool as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "enum.defined_only",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});
