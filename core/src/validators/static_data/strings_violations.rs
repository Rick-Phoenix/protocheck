use std::sync::LazyLock;

use proto_types::{
  field_descriptor_proto::Type as ProtoType,
  protovalidate::{FieldPath, FieldPathElement, Violation},
};

pub(crate) static STRING_LEN_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("string".to_string()),
      field_number: Some(14),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("len".to_string()),
      field_number: Some(19),
      field_type: Some(ProtoType::Uint64 as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_MIN_LEN_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("string".to_string()),
      field_number: Some(14),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("min_len".to_string()),
      field_number: Some(2),
      field_type: Some(ProtoType::Uint64 as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_MAX_LEN_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("string".to_string()),
      field_number: Some(14),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("max_len".to_string()),
      field_number: Some(3),
      field_type: Some(ProtoType::Uint64 as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_LEN_BYTES_VIOLATION: LazyLock<Vec<FieldPathElement>> =
  LazyLock::new(|| {
    vec![
      FieldPathElement {
        field_name: Some("string".to_string()),
        field_number: Some(14),
        field_type: Some(ProtoType::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("len_bytes".to_string()),
        field_number: Some(20),
        field_type: Some(ProtoType::Uint64 as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });

pub(crate) static STRING_MIN_BYTES_VIOLATION: LazyLock<Vec<FieldPathElement>> =
  LazyLock::new(|| {
    vec![
      FieldPathElement {
        field_name: Some("string".to_string()),
        field_number: Some(14),
        field_type: Some(ProtoType::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("min_bytes".to_string()),
        field_number: Some(4),
        field_type: Some(ProtoType::Uint64 as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });

pub(crate) static STRING_MAX_BYTES_VIOLATION: LazyLock<Vec<FieldPathElement>> =
  LazyLock::new(|| {
    vec![
      FieldPathElement {
        field_name: Some("string".to_string()),
        field_number: Some(14),
        field_type: Some(ProtoType::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("max_bytes".to_string()),
        field_number: Some(5),
        field_type: Some(ProtoType::Uint64 as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });

pub(crate) static STRING_PATTERN_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("string".to_string()),
      field_number: Some(14),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("pattern".to_string()),
      field_number: Some(6),
      field_type: Some(ProtoType::String as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_PREFIX_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("string".to_string()),
      field_number: Some(14),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("prefix".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::String as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_SUFFIX_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("string".to_string()),
      field_number: Some(14),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("suffix".to_string()),
      field_number: Some(8),
      field_type: Some(ProtoType::String as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_CONTAINS_VIOLATION: LazyLock<Vec<FieldPathElement>> =
  LazyLock::new(|| {
    vec![
      FieldPathElement {
        field_name: Some("string".to_string()),
        field_number: Some(14),
        field_type: Some(ProtoType::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("contains".to_string()),
        field_number: Some(9),
        field_type: Some(ProtoType::String as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });

pub(crate) static STRING_NOT_CONTAINS_VIOLATION: LazyLock<Vec<FieldPathElement>> =
  LazyLock::new(|| {
    vec![
      FieldPathElement {
        field_name: Some("string".to_string()),
        field_number: Some(14),
        field_type: Some(ProtoType::Message as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      },
      FieldPathElement {
        field_name: Some("not_contains".to_string()),
        field_number: Some(23),
        field_type: Some(ProtoType::String as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });
