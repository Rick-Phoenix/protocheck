use proto_types::protovalidate::Ignore;
use regex::Regex;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, Violation},
  validators::static_data::{
    base_violations::{get_base_violations_path, get_violation_elements},
    strings_violations::{
      STRING_ADDRESS_VIOLATION, STRING_CONTAINS_VIOLATION, STRING_EMAIL_VIOLATION,
      STRING_HOSTNAME_VIOLATION, STRING_HOST_AND_PORT_VIOLATION, STRING_IPV4_PREFIX_VIOLATION,
      STRING_IPV4_VIOLATION, STRING_IPV4_WITH_PREFIX_LEN_VIOLATION, STRING_IPV6_PREFIX_VIOLATION,
      STRING_IPV6_VIOLATION, STRING_IPV6_WITH_PREFIX_LEN_VIOLATION, STRING_IP_PREFIX_VIOLATION,
      STRING_IP_VIOLATION, STRING_IP_WITH_PREFIX_LEN_VIOLATION, STRING_LEN_BYTES_VIOLATION,
      STRING_LEN_VIOLATION, STRING_MAX_BYTES_VIOLATION, STRING_MAX_LEN_VIOLATION,
      STRING_MIN_BYTES_VIOLATION, STRING_MIN_LEN_VIOLATION, STRING_NOT_CONTAINS_VIOLATION,
      STRING_PATTERN_VIOLATION, STRING_PREFIX_VIOLATION, STRING_SUFFIX_VIOLATION,
      STRING_TUUID_VIOLATION, STRING_URI_REF_VIOLATION, STRING_URI_VIOLATION,
      STRING_UUID_VIOLATION, STRING_WELL_KNOWN_REGEX_VIOLATION,
    },
    well_known_strings::{
      is_valid_address, is_valid_email, is_valid_host_and_port, is_valid_hostname,
      is_valid_http_header_name, is_valid_http_header_value, is_valid_ip, is_valid_ip_prefix,
      is_valid_ip_with_prefixlen, is_valid_ipv4, is_valid_ipv4_prefix,
      is_valid_ipv4_with_prefixlen, is_valid_ipv6, is_valid_ipv6_prefix,
      is_valid_ipv6_with_prefixlen, is_valid_trimmed_uuid, is_valid_uri, is_valid_uri_ref,
      is_valid_uuid,
    },
  },
};

pub fn header_name(
  field_context: &FieldContext,
  value: &str,
  strict: bool,
) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_http_header_name(value, strict);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_WELL_KNOWN_REGEX_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.well_known_regex.header_name".to_string()),
      message: Some(format!(
        "{} must be a valid HTTP header name",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn header_value(
  field_context: &FieldContext,
  value: &str,
  strict: bool,
) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_http_header_value(value, strict);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_WELL_KNOWN_REGEX_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.well_known_regex.header_value".to_string()),
      message: Some(format!(
        "{} must be a valid HTTP header value",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn host_and_port(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_host_and_port(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_HOST_AND_PORT_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.host_and_port".to_string()),
      message: Some(format!(
        "{} must be a valid pair of host (hostname or IP address) and port",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn ip_prefix(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_ip_prefix(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_IP_PREFIX_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.ip_prefix".to_string()),
      message: Some(format!(
        "{} must be a valid ip prefix",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn ipv4_prefix(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_ipv4_prefix(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_IPV4_PREFIX_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.ipv4_prefix".to_string()),
      message: Some(format!(
        "{} must be a valid ipv4 prefix",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn ipv6_prefix(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_ipv6_prefix(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_IPV6_PREFIX_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.ipv6_prefix".to_string()),
      message: Some(format!(
        "{} must be a valid ipv6 prefix",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn ip_with_prefix_len(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_ip_with_prefixlen(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_IP_WITH_PREFIX_LEN_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.ip_with_prefixlen".to_string()),
      message: Some(format!(
        "{} must be a valid ip address with prefix length",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn ipv6_with_prefix_len(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_ipv6_with_prefixlen(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_IPV6_WITH_PREFIX_LEN_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.ipv6_with_prefixlen".to_string()),
      message: Some(format!(
        "{} must be a valid ipv6 address with prefix length",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn ipv4_with_prefix_len(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_ipv4_with_prefixlen(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_IPV4_WITH_PREFIX_LEN_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.ipv4_with_prefixlen".to_string()),
      message: Some(format!(
        "{} must be a valid ipv4 address with prefix length",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn tuuid(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_trimmed_uuid(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_TUUID_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.tuuid".to_string()),
      message: Some(format!(
        "{} must be a valid trimmed uuid",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn uuid(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_uuid(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_UUID_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.uuid".to_string()),
      message: Some(format!(
        "{} must be a valid uuid",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn address(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_address(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_ADDRESS_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.address".to_string()),
      message: Some(format!(
        "{} must be a valid hostname or ip address",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn uri_ref(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_uri_ref(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_URI_REF_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.uri_ref".to_string()),
      message: Some(format!(
        "{} must be a valid URI reference",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn uri(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_uri(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_URI_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.uri".to_string()),
      message: Some(format!(
        "{} must be a valid URI",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn hostname(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_hostname(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_HOSTNAME_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.hostname".to_string()),
      message: Some(format!(
        "{} must be a valid hostname",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn ip(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_ip(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_IP_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.ip".to_string()),
      message: Some(format!(
        "{} must be a valid ip address",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn ipv4(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_ipv4(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_IPV4_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.ipv4".to_string()),
      message: Some(format!(
        "{} must be a valid ipv4 address",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn ipv6(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_ipv6(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_IPV6_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.ipv6".to_string()),
      message: Some(format!(
        "{} must be a valid ipv6 address",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn email(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = is_valid_email(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_EMAIL_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.email".to_string()),
      message: Some(format!(
        "{} must be a valid email address",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn pattern(
  field_context: &FieldContext,
  value: &str,
  pattern: &Regex,
) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = pattern.is_match(value);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_PATTERN_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.pattern".to_string()),
      message: Some(format!(
        "{} match the following regex: `{}`",
        field_context.field_data.proto_name.clone(),
        pattern
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn contains(field_context: &FieldContext, value: &str, pattern: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.contains(pattern);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_CONTAINS_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.contains".to_string()),
      message: Some(format!(
        "{} must contain the '{}' substring",
        field_context.field_data.proto_name.clone(),
        pattern
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn not_contains(
  field_context: &FieldContext,
  value: &str,
  pattern: &str,
) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = !value.contains(pattern);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_NOT_CONTAINS_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.not_contains".to_string()),
      message: Some(format!(
        "{} must not contain the '{}' substring",
        field_context.field_data.proto_name.clone(),
        pattern
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn prefix(field_context: &FieldContext, value: &str, prefix: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.starts_with(prefix);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_PREFIX_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.prefix".to_string()),
      message: Some(format!(
        "{} must contain the '{}' prefix",
        field_context.field_data.proto_name.clone(),
        prefix
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn suffix(field_context: &FieldContext, value: &str, suffix: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.ends_with(suffix);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_SUFFIX_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.suffix".to_string()),
      message: Some(format!(
        "{} must contain the '{}' suffix",
        field_context.field_data.proto_name.clone(),
        suffix
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn max_len(field_context: &FieldContext, value: &str, max_len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.chars().count() <= max_len as usize;

  let plural_suffix = if max_len > 1 { "s" } else { "" };

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_MAX_LEN_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.max_len".to_string()),
      message: Some(format!(
        "{} cannot be longer than {} character{}",
        field_context.field_data.proto_name.clone(),
        max_len,
        plural_suffix
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn min_len(field_context: &FieldContext, value: &str, min_len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.chars().count() >= min_len as usize;

  let plural_suffix = if min_len > 1 { "s" } else { "" };

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_MIN_LEN_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.min_len".to_string()),
      message: Some(format!(
        "{} cannot be shorter than {} character{}",
        field_context.field_data.proto_name.clone(),
        min_len,
        plural_suffix
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn len(field_context: &FieldContext, value: &str, len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.chars().count() == len as usize;

  let plural_suffix = if len > 1 { "s" } else { "" };

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_LEN_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.len".to_string()),
      message: Some(format!(
        "{} must be exactly {} character{} long",
        field_context.field_data.proto_name.clone(),
        len,
        plural_suffix
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn len_bytes(field_context: &FieldContext, value: &str, len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.len() == len as usize;

  let plural_suffix = if len > 1 { "s" } else { "" };

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_LEN_BYTES_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.len_bytes".to_string()),
      message: Some(format!(
        "{} must be exactly {} byte{} long",
        field_context.field_data.proto_name.clone(),
        len,
        plural_suffix
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn min_bytes(field_context: &FieldContext, value: &str, min_len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.len() >= min_len as usize;

  let plural_suffix = if min_len > 1 { "s" } else { "" };

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_MIN_BYTES_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.min_bytes".to_string()),
      message: Some(format!(
        "{} cannot be shorter than {} bytes{}",
        field_context.field_data.proto_name.clone(),
        min_len,
        plural_suffix
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn max_bytes(field_context: &FieldContext, value: &str, max_len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.len() <= max_len as usize;

  let plural_suffix = if max_len > 1 { "s" } else { "" };

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_data.kind);

    rule_elements.extend(STRING_MAX_BYTES_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("string.max_bytes".to_string()),
      message: Some(format!(
        "{} cannot be longer than {} byte{}",
        field_context.field_data.proto_name.clone(),
        max_len,
        plural_suffix
      )),
      for_key: Some(field_context.field_data.kind.is_map_key()),
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}
