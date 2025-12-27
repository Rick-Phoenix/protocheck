use super::*;

pub static FLOAT_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("float".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Float as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "float.const",
  }
});

pub static DOUBLE_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("double".to_string()),
      field_number: Some(2),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Double as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "double.const",
  }
});

pub static INT32_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("int32".to_string()),
      field_number: Some(3),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Int32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "int32.const",
  }
});

pub static INT64_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("int64".to_string()),
      field_number: Some(4),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Int64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "int64.const",
  }
});

pub static UINT32_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("uint32".to_string()),
      field_number: Some(5),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Uint32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "uint32.const",
  }
});

pub static UINT64_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("uint64".to_string()),
      field_number: Some(6),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Uint64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "uint64.const",
  }
});

pub static SINT32_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("sint32".to_string()),
      field_number: Some(7),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Sint32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "sint32.const",
  }
});

pub static SINT64_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("sint64".to_string()),
      field_number: Some(8),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Sint64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "sint64.const",
  }
});

pub static FIXED32_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("fixed32".to_string()),
      field_number: Some(9),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Fixed32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "fixed32.const",
  }
});

pub static FIXED64_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("fixed64".to_string()),
      field_number: Some(10),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Fixed64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "fixed64.const",
  }
});

pub static SFIXED32_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("sfixed32".to_string()),
      field_number: Some(11),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Sfixed32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "sfixed32.const",
  }
});

pub static SFIXED64_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("sfixed64".to_string()),
      field_number: Some(12),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Sfixed64 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "sfixed64.const",
  }
});

pub static BOOL_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("bool".to_string()),
      field_number: Some(13),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Bool as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "bool.const",
  }
});

pub static STRING_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("string".to_string()),
      field_number: Some(14),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::String as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "string.const",
  }
});

pub static BYTES_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("bytes".to_string()),
      field_number: Some(15),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Bytes as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "bytes.const",
  }
});

pub static ENUM_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("enum".to_string()),
      field_number: Some(16),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Int32 as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "enum.const",
  }
});

pub static DURATION_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("duration".to_string()),
      field_number: Some(21),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(2),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "duration.const",
  }
});

pub static TIMESTAMP_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("timestamp".to_string()),
      field_number: Some(22),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(2),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    elements: Box::leak(elements.into_boxed_slice()),
    name: "timestamp.const",
  }
});

pub static FIELD_MASK_CONST_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let elements = vec![
    FieldPathElement {
      field_name: Some("field_mask".to_string()),
      field_number: Some(28),
      field_type: Some(Type::Message as i32),
      ..Default::default()
    },
    FieldPathElement {
      field_name: Some("const".to_string()),
      field_number: Some(1),
      field_type: Some(Type::Bool as i32),
      ..Default::default()
    },
  ];

  ViolationData {
    name: "field_mask.const",
    elements: Box::leak(elements.into_boxed_slice()),
  }
});
