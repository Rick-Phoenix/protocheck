use super::*;

pub static REPEATED_MIN_ITEMS_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("repeated".to_string()),
      field_number: Some(18),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("min_items".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Uint64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "repeated.min_items",
  }
});

pub static REPEATED_MAX_ITEMS_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("repeated".to_string()),
      field_number: Some(18),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("max_items".to_string()),
      field_number: Some(2),
      field_type: Some(Type::Uint64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "repeated.max_items",
  }
});

pub static REPEATED_UNIQUE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("repeated".to_string()),
      field_number: Some(18),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("unique".to_string()),
      field_number: Some(3),
      field_type: Some(Type::Bool as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "repeated.unique",
  }
});

pub static REPEATED_ITEM_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("repeated".to_string()),
      field_number: Some(18),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("items".to_string()),
      field_number: Some(4),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(violation.into_boxed_slice()),
    name: "repeated.items",
  }
});
