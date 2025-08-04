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

pub(crate) static STRING_EMAIL_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
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
      field_name: Some("email".to_string()),
      field_number: Some(12),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_HOSTNAME_VIOLATION: LazyLock<Vec<FieldPathElement>> =
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
        field_name: Some("hostname".to_string()),
        field_number: Some(13),
        field_type: Some(ProtoType::Bool as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });

pub(crate) static STRING_IP_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
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
      field_name: Some("ip".to_string()),
      field_number: Some(14),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_IPV4_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
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
      field_name: Some("ipv4".to_string()),
      field_number: Some(15),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_IPV6_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
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
      field_name: Some("ipv6".to_string()),
      field_number: Some(16),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_URI_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
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
      field_name: Some("uri".to_string()),
      field_number: Some(17),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_URI_REF_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
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
      field_name: Some("uri_ref".to_string()),
      field_number: Some(18),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_ADDRESS_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
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
      field_name: Some("address".to_string()),
      field_number: Some(21),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_UUID_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
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
      field_name: Some("uuid".to_string()),
      field_number: Some(22),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_TUUID_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
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
      field_name: Some("tuuid".to_string()),
      field_number: Some(33),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

pub(crate) static STRING_IP_WITH_PREFIX_LEN_VIOLATION: LazyLock<Vec<FieldPathElement>> =
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
        field_name: Some("ip_with_prefixlen".to_string()),
        field_number: Some(26),
        field_type: Some(ProtoType::Bool as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });

pub(crate) static STRING_IPV4_WITH_PREFIX_LEN_VIOLATION: LazyLock<Vec<FieldPathElement>> =
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
        field_name: Some("ipv4_with_prefixlen".to_string()),
        field_number: Some(27),
        field_type: Some(ProtoType::Bool as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });

pub(crate) static STRING_IPV6_WITH_PREFIX_LEN_VIOLATION: LazyLock<Vec<FieldPathElement>> =
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
        field_name: Some("ipv6_with_prefixlen".to_string()),
        field_number: Some(28),
        field_type: Some(ProtoType::Bool as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });

pub(crate) static STRING_IP_PREFIX_VIOLATION: LazyLock<Vec<FieldPathElement>> =
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
        field_name: Some("ip_prefix".to_string()),
        field_number: Some(29),
        field_type: Some(ProtoType::Bool as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });

pub(crate) static STRING_IPV4_PREFIX_VIOLATION: LazyLock<Vec<FieldPathElement>> =
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
        field_name: Some("ipv4_prefix".to_string()),
        field_number: Some(30),
        field_type: Some(ProtoType::Bool as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });

pub(crate) static STRING_IPV6_PREFIX_VIOLATION: LazyLock<Vec<FieldPathElement>> =
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
        field_name: Some("ipv6_prefix".to_string()),
        field_number: Some(31),
        field_type: Some(ProtoType::Bool as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });

pub(crate) static STRING_HOST_AND_PORT_VIOLATION: LazyLock<Vec<FieldPathElement>> =
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
        field_name: Some("host_and_port".to_string()),
        field_number: Some(32),
        field_type: Some(ProtoType::Bool as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });

pub(crate) static STRING_WELL_KNOWN_REGEX_VIOLATION: LazyLock<Vec<FieldPathElement>> =
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
        field_name: Some("well_known_regex".to_string()),
        field_number: Some(24),
        field_type: Some(ProtoType::Enum as i32),
        key_type: None,
        value_type: None,
        subscript: None,
      },
    ]
  });
