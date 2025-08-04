use proto_types::protovalidate::Ignore;
use regex::Regex;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, Violation},
  validators::static_data::{
    base_violations::{get_base_violations_path, get_violation_elements},
    strings_violations::{
      STRING_CONTAINS_VIOLATION, STRING_LEN_BYTES_VIOLATION, STRING_LEN_VIOLATION,
      STRING_MAX_BYTES_VIOLATION, STRING_MAX_LEN_VIOLATION, STRING_MIN_BYTES_VIOLATION,
      STRING_MIN_LEN_VIOLATION, STRING_NOT_CONTAINS_VIOLATION, STRING_PATTERN_VIOLATION,
      STRING_PREFIX_VIOLATION, STRING_SUFFIX_VIOLATION,
    },
  },
};

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
