use std::{str::from_utf8, sync::LazyLock};

use bytes::Bytes;
use proto_types::{
  field_descriptor_proto::Type as ProtoType,
  protovalidate::{FieldPath, FieldPathElement, Violation},
};

use crate::{
  field_data::FieldContext, validators::static_data::base_violations::get_violation_elements,
};

fn get_invalid_bytes_violation(elements: Vec<FieldPathElement>) -> Violation {
  Violation {
    rule_id: Some("utf8_error".to_string()),
    message: Some("invalid utf8 bytes".to_string()),
    field: Some(FieldPath { elements }),
    rule: None,
    for_key: None,
  }
}

pub(crate) fn parse_bytes_input<'a>(
  value: &'a Bytes,
  field_context: &'a FieldContext<'a>,
) -> Result<&'a str, Violation> {
  from_utf8(value).map_err(|_| {
    let elements = get_violation_elements(field_context);
    get_invalid_bytes_violation(elements)
  })
}

pub(crate) static BYTES_LEN_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("bytes".to_string()),
      field_number: Some(15),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("len".to_string()),
      field_number: Some(13),
      field_type: Some(ProtoType::Uint64 as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static BYTES_MIN_LEN_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("bytes".to_string()),
      field_number: Some(15),
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

pub(crate) static BYTES_MAX_LEN_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("bytes".to_string()),
      field_number: Some(15),
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

pub(crate) static BYTES_PATTERN_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("bytes".to_string()),
      field_number: Some(15),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("pattern".to_string()),
      field_number: Some(6),
      field_type: Some(ProtoType::Bytes as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static BYTES_PREFIX_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("bytes".to_string()),
      field_number: Some(15),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("prefix".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Bytes as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static BYTES_SUFFIX_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("bytes".to_string()),
      field_number: Some(15),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("suffix".to_string()),
      field_number: Some(6),
      field_type: Some(ProtoType::Bytes as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static BYTES_CONTAINS_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("bytes".to_string()),
      field_number: Some(15),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("contains".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Bytes as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

#[cfg(feature = "ip")]
pub(crate) static BYTES_IP_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("bytes".to_string()),
      field_number: Some(15),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("ip".to_string()),
      field_number: Some(10),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

#[cfg(feature = "ip")]
pub(crate) static BYTES_IPV4_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("bytes".to_string()),
      field_number: Some(15),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("ipv4".to_string()),
      field_number: Some(11),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

#[cfg(feature = "ip")]
pub(crate) static BYTES_IPV6_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("bytes".to_string()),
      field_number: Some(15),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("ipv6".to_string()),
      field_number: Some(12),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});
