use std::sync::LazyLock;

use super::*;
use crate::{protovalidate::FieldPathElement, ProtoType};

pub(crate) static FLOAT_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Float as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "float.gt",
  }
});

pub(crate) static DOUBLE_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Double as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "double.gt",
  }
});

pub(crate) static INT32_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Int32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "int32.gt",
  }
});

pub(crate) static INT64_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Int64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "int64.gt",
  }
});

pub(crate) static UINT32_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Uint32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "uint32.gt",
  }
});

pub(crate) static UINT64_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Uint64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "uint64.gt",
  }
});

pub(crate) static SINT32_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Sint32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "sint32.gt",
  }
});

pub(crate) static SINT64_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Sint64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "sint64.gt",
  }
});

pub(crate) static FIXED32_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Fixed32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "fixed32.gt",
  }
});

pub(crate) static FIXED64_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Fixed64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "fixed64.gt",
  }
});

pub(crate) static SFIXED32_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Sfixed32 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "sfixed32.gt",
  }
});

pub(crate) static SFIXED64_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Sfixed64 as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "sfixed64.gt",
  }
});

pub(crate) static DURATION_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "duration.gt",
  }
});

pub(crate) static TIMESTAMP_GT_VIOLATION: LazyLock<ViolationData> = LazyLock::new(|| {
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
      field_name: Some("gt".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ];

  ViolationData {
    violation,
    name: "timestamp.gt",
  }
});
