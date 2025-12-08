use std::sync::LazyLock;

use super::*;
use crate::{protovalidate::FieldPathElement, ProtoType};

pub(crate) static ANY_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("any".to_string()),
      field_number: Some(20),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(3),
      field_type: Some(ProtoType::String as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "any.not_in",
    violation,
  }
});

pub(crate) static FLOAT_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("float".to_string()),
      field_number: Some(1),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Float as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "float.not_in",
    violation,
  }
});

pub(crate) static DOUBLE_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("double".to_string()),
      field_number: Some(2),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Double as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "double.not_in",
    violation,
  }
});

pub(crate) static INT32_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("int32".to_string()),
      field_number: Some(3),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Int32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "int32.not_in",
    violation,
  }
});

pub(crate) static INT64_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("int64".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Int64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "int64.not_in",
    violation,
  }
});

pub(crate) static UINT32_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("uint32".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Uint32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "uint32.not_in",
    violation,
  }
});

pub(crate) static UINT64_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("uint64".to_string()),
      field_number: Some(6),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Uint64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "uint64.not_in",
    violation,
  }
});

pub(crate) static SINT32_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("sint32".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Sint32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "sint32.not_in",
    violation,
  }
});

pub(crate) static SINT64_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("sint64".to_string()),
      field_number: Some(8),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Sint64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "sint64.not_in",
    violation,
  }
});

pub(crate) static FIXED32_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("fixed32".to_string()),
      field_number: Some(9),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Fixed32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "fixed32.not_in",
    violation,
  }
});

pub(crate) static FIXED64_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("fixed64".to_string()),
      field_number: Some(10),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Fixed64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "fixed64.not_in",
    violation,
  }
});

pub(crate) static SFIXED32_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("sfixed32".to_string()),
      field_number: Some(11),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Sfixed32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "sfixed32.not_in",
    violation,
  }
});

pub(crate) static SFIXED64_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("sfixed64".to_string()),
      field_number: Some(12),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Sfixed64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "sfixed64.not_in",
    violation,
  }
});

pub(crate) static STRING_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("string".to_string()),
      field_number: Some(14),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(11),
      field_type: Some(ProtoType::String as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "string.not_in",
    violation,
  }
});

#[cfg(feature = "bytes")]
pub(crate) static BYTES_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("bytes".to_string()),
      field_number: Some(15),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(9),
      field_type: Some(ProtoType::Bytes as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "bytes.not_in",
    violation,
  }
});

pub(crate) static ENUM_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("enum".to_string()),
      field_number: Some(16),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Int32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "enum.not_in",
    violation,
  }
});

pub(crate) static DURATION_NOT_IN_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("duration".to_string()),
      field_number: Some(21),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("not_in".to_string()),
      field_number: Some(8),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    name: "duration.not_in",
    violation,
  }
});
