use prost::bytes::Bytes;
use regex::Regex;

use crate::{
  field_data::FieldContext,
  protovalidate::Violation,
  validators::static_data::{
    base_violations::create_violation,
    bytes_violations::{
      parse_bytes_input, BYTES_CONTAINS_VIOLATION, BYTES_IPV4_VIOLATION, BYTES_IPV6_VIOLATION,
      BYTES_IP_VIOLATION, BYTES_LEN_VIOLATION, BYTES_MAX_LEN_VIOLATION, BYTES_MIN_LEN_VIOLATION,
      BYTES_PATTERN_VIOLATION, BYTES_PREFIX_VIOLATION, BYTES_SUFFIX_VIOLATION,
    },
    well_known_strings::{is_valid_ip, is_valid_ipv4, is_valid_ipv6},
  },
};

pub fn ip(field_context: &FieldContext, value: &Bytes) -> Result<(), Violation> {
  let string_val = parse_bytes_input(value, field_context)?;
  let check = is_valid_ip(string_val);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &BYTES_IP_VIOLATION,
      "bytes.ip",
      "must be a valid ip address",
    ))
  }
}

pub fn ipv4(field_context: &FieldContext, value: &Bytes) -> Result<(), Violation> {
  let string_val = parse_bytes_input(value, field_context)?;
  let check = is_valid_ipv4(string_val);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &BYTES_IPV4_VIOLATION,
      "bytes.ipv4",
      "must be a valid ipv4 address",
    ))
  }
}

pub fn ipv6(field_context: &FieldContext, value: &Bytes) -> Result<(), Violation> {
  let string_val = parse_bytes_input(value, field_context)?;
  let check = is_valid_ipv6(string_val);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &BYTES_IPV6_VIOLATION,
      "bytes.ipv6",
      "must be a valid ipv6 address",
    ))
  }
}

pub fn pattern(
  field_context: &FieldContext,
  value: &Bytes,
  pattern: &Regex,
  error_message: &'static str,
) -> Result<(), Violation> {
  let string_val = parse_bytes_input(value, field_context)?;

  let check = pattern.is_match(string_val);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &BYTES_PATTERN_VIOLATION,
      "bytes.pattern",
      error_message,
    ))
  }
}

pub fn contains(
  field_context: &FieldContext,
  value: &Bytes,
  pattern: &'static [u8],
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = value.windows(pattern.len()).any(|win| win == pattern);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &BYTES_CONTAINS_VIOLATION,
      "bytes.contains",
      error_message,
    ))
  }
}

pub fn suffix(
  field_context: &FieldContext,
  value: &Bytes,
  suffix: &'static [u8],
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = value.ends_with(suffix);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &BYTES_SUFFIX_VIOLATION,
      "bytes.suffix",
      error_message,
    ))
  }
}

pub fn prefix(
  field_context: &FieldContext,
  value: &Bytes,
  prefix: &'static [u8],
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = value.starts_with(prefix);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &BYTES_PREFIX_VIOLATION,
      "bytes.prefix",
      error_message,
    ))
  }
}

pub fn max_len(
  field_context: &FieldContext,
  value: &Bytes,
  max_len: u64,
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = value.len() <= max_len as usize;

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &BYTES_MAX_LEN_VIOLATION,
      "bytes.max_len",
      error_message,
    ))
  }
}

pub fn min_len(
  field_context: &FieldContext,
  value: &Bytes,
  min_len: u64,
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = value.len() >= min_len as usize;

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &BYTES_MIN_LEN_VIOLATION,
      "bytes.min_len",
      error_message,
    ))
  }
}

pub fn len(
  field_context: &FieldContext,
  value: &Bytes,
  len: u64,
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = value.len() == len as usize;

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &BYTES_LEN_VIOLATION,
      "bytes.len",
      error_message,
    ))
  }
}
