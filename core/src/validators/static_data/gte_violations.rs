use std::sync::LazyLock;

use super::*;
use crate::{protovalidate::FieldPathElement, ProtoType};

pub(crate) static FLOAT_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Float as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "float.gte",
  }
});

pub(crate) static DOUBLE_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Double as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "double.gte",
  }
});

pub(crate) static INT32_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Int32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "int32.gte",
  }
});

pub(crate) static INT64_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Int64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "int64.gte",
  }
});

pub(crate) static UINT32_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Uint32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "uint32.gte",
  }
});

pub(crate) static UINT64_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Uint64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "uint64.gte",
  }
});

pub(crate) static SINT32_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Sint32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "sint32.gte",
  }
});

pub(crate) static SINT64_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Sint64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "sint64.gte",
  }
});

pub(crate) static FIXED32_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Fixed32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "fixed32.gte",
  }
});

pub(crate) static FIXED64_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Fixed64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "fixed64.gte",
  }
});

pub(crate) static SFIXED32_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Sfixed32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "sfixed32.gte",
  }
});

pub(crate) static SFIXED64_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gte".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Sfixed64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "sfixed64.gte",
  }
});

pub(crate) static DURATION_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gte".to_string()),
      field_number: Some(6),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "duration.gte",
  }
});

pub(crate) static TIMESTAMP_GTE_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
  let violation = vec![
    FieldPathElement {
      field_name: Some("timestamp".to_string()),
      field_number: Some(22),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gte".to_string()),
      field_number: Some(6),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "timestamp.gte",
  }
});
