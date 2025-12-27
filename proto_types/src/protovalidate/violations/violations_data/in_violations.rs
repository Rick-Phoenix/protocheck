use super::*;

pub static ANY_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("any".to_string()),
      field_number: Some(20),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(2),
      field_type: Some(Type::String as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "any.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static FLOAT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("float".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Float as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "float.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static DOUBLE_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("double".to_string()),
      field_number: Some(2),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Double as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "double.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static INT32_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("int32".to_string()),
      field_number: Some(3),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Int32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "int32.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static INT64_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("int64".to_string()),
      field_number: Some(4),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Int64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "int64.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static UINT32_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("uint32".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Uint32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "uint32.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static UINT64_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("uint64".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Uint64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "uint64.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static SINT32_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("sint32".to_string()),
      field_number: Some(7),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Sint32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "sint32.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static SINT64_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("sint64".to_string()),
      field_number: Some(8),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Sint64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "sint64.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static FIXED32_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("fixed32".to_string()),
      field_number: Some(9),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Fixed32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "fixed32.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static FIXED64_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("fixed64".to_string()),
      field_number: Some(10),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Fixed64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "fixed64.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static SFIXED32_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("sfixed32".to_string()),
      field_number: Some(11),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Sfixed32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "sfixed32.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static SFIXED64_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("sfixed64".to_string()),
      field_number: Some(12),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Sfixed64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "sfixed64.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static STRING_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("string".to_string()),
      field_number: Some(14),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(10),
      field_type: Some(Type::String as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "string.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static BYTES_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("bytes".to_string()),
      field_number: Some(15),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(8),
      field_type: Some(Type::Bytes as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "bytes.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static ENUM_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("enum".to_string()),
      field_number: Some(16),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(3),
      field_type: Some(Type::Int32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "enum.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static DURATION_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("duration".to_string()),
      field_number: Some(21),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(7),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "duration.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});

pub static FIELD_MASK_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("field_mask".to_string()),
      field_number: Some(28),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("in".to_string()),
      field_number: Some(2),
      field_type: Some(Type::String as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "field_mask.in",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});
