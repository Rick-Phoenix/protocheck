use super::*;

pub static ONEOF_REQUIRED_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![FieldPathElement {
    field_name: Some("required".to_string()),
    field_number: Some(1),
    field_type: Some(Type::Bool as i32),
    ..Default::default()
  }];

  ViolationData {
    name: "oneof.required",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});
