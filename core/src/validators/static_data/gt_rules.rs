use std::sync::LazyLock;

use proto_types::FieldType;

use crate::{protovalidate::FieldPathElement, ProtoType};

pub(crate) fn get_gt_rule_path(
  kind: FieldType,
) -> Option<(&'static str, &'static [FieldPathElement])> {
  match kind {
    FieldType::Float => Some(("float", &FLOAT_GT_VIOLATION)),
    FieldType::Double => Some(("double", &DOUBLE_GT_VIOLATION)),
    FieldType::Int32 => Some(("int32", &INT32_GT_VIOLATION)),
    FieldType::Int64 => Some(("int64", &INT64_GT_VIOLATION)),
    FieldType::Uint32 => Some(("uint32", &UINT32_GT_VIOLATION)),
    FieldType::Uint64 => Some(("uint64", &UINT64_GT_VIOLATION)),
    FieldType::Sint32 => Some(("sint32", &SINT32_GT_VIOLATION)),
    FieldType::Sint64 => Some(("sint64", &SINT64_GT_VIOLATION)),
    FieldType::Fixed32 => Some(("fixed32", &FIXED32_GT_VIOLATION)),
    FieldType::Fixed64 => Some(("fixed64", &FIXED64_GT_VIOLATION)),
    FieldType::Sfixed32 => Some(("sfixed32", &SFIXED32_GT_VIOLATION)),
    FieldType::Sfixed64 => Some(("sfixed64", &SFIXED64_GT_VIOLATION)),
    FieldType::Duration => Some(("duration", &DURATION_GT_VIOLATION)),
    FieldType::Timestamp => Some(("timestamp", &TIMESTAMP_GT_VIOLATION)),
    _ => None,
  }
}

static FLOAT_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("float".to_string()),
      field_number: Some(1),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Float as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static DOUBLE_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("double".to_string()),
      field_number: Some(2),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Double as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static INT32_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("int32".to_string()),
      field_number: Some(3),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Int32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static INT64_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("int64".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Int64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static UINT32_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("uint32".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Uint32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static UINT64_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("uint64".to_string()),
      field_number: Some(6),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Uint64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static SINT32_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("sint32".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Sint32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static SINT64_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("sint64".to_string()),
      field_number: Some(8),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Sint64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static FIXED32_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("fixed32".to_string()),
      field_number: Some(9),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Fixed32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static FIXED64_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("fixed64".to_string()),
      field_number: Some(10),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Fixed64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static SFIXED32_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("sfixed32".to_string()),
      field_number: Some(11),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Sfixed32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static SFIXED64_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("sfixed64".to_string()),
      field_number: Some(12),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Sfixed64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static DURATION_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("duration".to_string()),
      field_number: Some(21),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static TIMESTAMP_GT_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("timestamp".to_string()),
      field_number: Some(22),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});
