use super::*;

pub static CEL_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![FieldPathElement {
    field_name: Some("cel".to_string()),
    field_number: Some(23),
    field_type: Some(Type::Message as i32),
    ..Default::default()
  }];

  ViolationData {
    name: "cel",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static REQUIRED_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![FieldPathElement {
    field_type: Some(Type::Bool as i32),
    field_name: Some("required".to_string()),
    field_number: Some(25),
    ..Default::default()
  }];

  ViolationData {
    name: "required",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});
