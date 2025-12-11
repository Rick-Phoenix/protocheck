use super::*;

pub static TIMESTAMP_WITHIN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("timestamp".to_string()),
      field_number: Some(22),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("within".to_string()),
      field_number: Some(9),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "timestamp.within",
  }
});

pub static TIMESTAMP_GT_NOW_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("timestamp".to_string()),
      field_number: Some(22),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gt_now".to_string()),
      field_number: Some(8),
      field_type: Some(Type::Bool as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "timestamp.gt_now",
  }
});

pub static TIMESTAMP_LT_NOW_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("timestamp".to_string()),
      field_number: Some(22),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("lt_now".to_string()),
      field_number: Some(7),
      field_type: Some(Type::Bool as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "timestamp.lt_now",
  }
});
