use super::*;

pub static FLOAT_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("float".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Float as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "float.gte",
  }
});

pub static DOUBLE_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("double".to_string()),
      field_number: Some(2),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Double as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "double.gte",
  }
});

pub static INT32_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("int32".to_string()),
      field_number: Some(3),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Int32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "int32.gte",
  }
});

pub static INT64_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("int64".to_string()),
      field_number: Some(4),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Int64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "int64.gte",
  }
});

pub static UINT32_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("uint32".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Uint32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "uint32.gte",
  }
});

pub static UINT64_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("uint64".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Uint64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "uint64.gte",
  }
});

pub static SINT32_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("sint32".to_string()),
      field_number: Some(7),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Sint32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "sint32.gte",
  }
});

pub static SINT64_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("sint64".to_string()),
      field_number: Some(8),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Sint64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "sint64.gte",
  }
});

pub static FIXED32_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("fixed32".to_string()),
      field_number: Some(9),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Fixed32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "fixed32.gte",
  }
});

pub static FIXED64_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("fixed64".to_string()),
      field_number: Some(10),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Fixed64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "fixed64.gte",
  }
});

pub static SFIXED32_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("sfixed32".to_string()),
      field_number: Some(11),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Sfixed32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "sfixed32.gte",
  }
});

pub static SFIXED64_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("sfixed64".to_string()),
      field_number: Some(12),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Sfixed64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "sfixed64.gte",
  }
});

pub static DURATION_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("duration".to_string()),
      field_number: Some(21),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "duration.gte",
  }
});

pub static TIMESTAMP_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("timestamp".to_string()),
      field_number: Some(22),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "timestamp.gte",
  }
});
