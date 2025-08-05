use prost::bytes::Bytes;
use proto_types::protovalidate::Ignore;
use regex::Regex;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, Violation},
  validators::static_data::{
    base_violations::{get_base_violations_path, get_violation_elements},
    bytes_violations::{
      parse_bytes_input, BYTES_CONTAINS_VIOLATION, BYTES_IPV4_VIOLATION, BYTES_IPV6_VIOLATION,
      BYTES_IP_VIOLATION, BYTES_LEN_VIOLATION, BYTES_MAX_LEN_VIOLATION, BYTES_MIN_LEN_VIOLATION,
      BYTES_PATTERN_VIOLATION, BYTES_PREFIX_VIOLATION, BYTES_SUFFIX_VIOLATION,
    },
    well_known_strings::{is_valid_ip, is_valid_ipv4, is_valid_ipv6},
  },
};

fn format_bytes_for_error(bytes: &[u8]) -> String {
  match std::str::from_utf8(bytes) {
    Ok(s) => s.to_string(),
    Err(_) => {
      let hex_string: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
      format!("0x{}", hex_string)
    }
  }
}

pub fn ip(field_context: &FieldContext, value: &Bytes) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let string_val = parse_bytes_input(value, field_context)?;
  let check = is_valid_ip(string_val);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_kind);

    rule_elements.extend(BYTES_IP_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("bytes.ip".to_string()),
      message: Some(format!(
        "{} must be a valid ip address",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn ipv4(field_context: &FieldContext, value: &Bytes) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let string_val = parse_bytes_input(value, field_context)?;
  let check = is_valid_ipv4(string_val);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_kind);

    rule_elements.extend(BYTES_IPV4_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("bytes.ipv4".to_string()),
      message: Some(format!(
        "{} must be a valid ipv4 address",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn ipv6(field_context: &FieldContext, value: &Bytes) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let string_val = parse_bytes_input(value, field_context)?;
  let check = is_valid_ipv6(string_val);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_kind);

    rule_elements.extend(BYTES_IPV6_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("bytes.ipv6".to_string()),
      message: Some(format!(
        "{} must be a valid ipv6 address",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: None,
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
  value: &Bytes,
  pattern: &Regex,
) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let string_val = parse_bytes_input(value, field_context)?;

  let check = pattern.is_match(string_val);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_kind);

    rule_elements.extend(BYTES_PATTERN_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("bytes.pattern".to_string()),
      message: Some(format!(
        "{} match the following regex: `{}`",
        field_context.field_data.proto_name.clone(),
        pattern
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: rule_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn contains(
  field_context: &FieldContext,
  value: &Bytes,
  pattern: &'static [u8],
) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.windows(pattern.len()).any(|win| win == pattern);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut violation_elements = get_base_violations_path(&field_context.field_kind);

    violation_elements.extend(BYTES_CONTAINS_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("bytes.contains".to_string()),
      message: Some(format!(
        "{} must contain '{}'",
        field_context.field_data.proto_name.clone(),
        format_bytes_for_error(pattern)
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn suffix(
  field_context: &FieldContext,
  value: &Bytes,
  suffix: &'static [u8],
) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.ends_with(suffix);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut violation_elements = get_base_violations_path(&field_context.field_kind);

    violation_elements.extend(BYTES_SUFFIX_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("bytes.suffix".to_string()),
      message: Some(format!(
        "{} must end with '{}'",
        field_context.field_data.proto_name.clone(),
        format_bytes_for_error(suffix)
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn prefix(
  field_context: &FieldContext,
  value: &Bytes,
  prefix: &'static [u8],
) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.starts_with(prefix);

  if !check {
    let elements = get_violation_elements(field_context);

    let mut violation_elements = get_base_violations_path(&field_context.field_kind);

    violation_elements.extend(BYTES_PREFIX_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("bytes.prefix".to_string()),
      message: Some(format!(
        "{} must start with '{}'",
        field_context.field_data.proto_name.clone(),
        format_bytes_for_error(prefix)
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}

pub fn max_len(field_context: &FieldContext, value: &Bytes, max_len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.field_data.ignore {
    if value.is_empty() {
      return Ok(());
    }
  }

  let check = value.len() <= max_len as usize;

  let plural_suffix = if max_len > 1 { "s" } else { "" };

  if !check {
    let elements = get_violation_elements(field_context);

    let mut rule_elements = get_base_violations_path(&field_context.field_kind);

    rule_elements.extend(BYTES_MAX_LEN_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("bytes.max_len".to_string()),
      message: Some(format!(
        "{} cannot be longer than {} byte{}",
        field_context.field_data.proto_name.clone(),
        max_len,
        plural_suffix
      )),
      for_key: None,
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

    let mut violation_elements = get_base_violations_path(&field_context.field_kind);

    violation_elements.extend(BYTES_MIN_LEN_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("bytes.min_len".to_string()),
      message: Some(format!(
        "{} cannot be shorter than {} byte{}",
        field_context.field_data.proto_name.clone(),
        min_len,
        plural_suffix
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
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

    let mut violation_elements = get_base_violations_path(&field_context.field_kind);

    violation_elements.extend(BYTES_LEN_VIOLATION.clone());

    let violation = Violation {
      rule_id: Some("bytes.len".to_string()),
      message: Some(format!(
        "{} must be exactly {} byte{} long",
        field_context.field_data.proto_name.clone(),
        len,
        plural_suffix
      )),
      for_key: None,
      field: Some(FieldPath { elements }),
      rule: Some(FieldPath {
        elements: violation_elements,
      }),
    };
    return Err(violation);
  };
  Ok(())
}
