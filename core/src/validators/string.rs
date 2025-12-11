use super::{well_known_strings::*, *};
use crate::protovalidate::violations_data::string_violations::*;

macro_rules! well_known_rule {
  (
    $name:ident,
    $definition:literal
  ) => {
    paste::paste! {
      pub fn $name(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
        let is_valid = [<is_valid _ $name>](value);

        if is_valid {
          Ok(())
        } else {
          Err(create_violation(
            field_context,
            &[< STRING _ $name:upper _ VIOLATION >],
            concat!("must be a valid ", $definition)
          ))
        }
      }
    }
  };
}

macro_rules! string_validator {
  (
    $name:ident,
    $target_type:ty,
    $validation_expression:expr
  ) => {
    paste::paste! {
      pub fn $name(
        field_context: &FieldContext,
        value: &str,
        target: $target_type,
        error_message: &str,
      ) -> Result<(), Violation> {
        let is_valid = ($validation_expression)(value, target);

        if is_valid {
          Ok(())
        } else {
          Err(create_violation(
            field_context,
            &[< STRING _ $name:upper _ VIOLATION >],
            error_message
          ))
        }
      }
    }
  };
}

// Char length
string_validator!(
  max_len,
  u64,
  |value: &str, max_len: u64| value.chars().count() as u64 <= max_len
);
string_validator!(
  min_len,
  u64,
  |value: &str, min_len: u64| value.chars().count() as u64 >= min_len
);
string_validator!(len, u64, |value: &str, max_len: u64| value.chars().count()
  as u64
  == max_len);

// Bytes length
string_validator!(len_bytes, u64, |value: &str, len: u64| value.len() as u64
  == len);
string_validator!(max_bytes, u64, |value: &str, max_bytes: u64| value.len()
  as u64
  <= max_bytes);
string_validator!(min_bytes, u64, |value: &str, min_bytes: u64| value.len()
  as u64
  >= min_bytes);

// Patterns
#[cfg(feature = "regex")]
string_validator!(
  pattern,
  &regex::Regex,
  |value: &str, regex: &regex::Regex| regex.is_match(value)
);
string_validator!(contains, &str, |value: &str, substring: &str| value
  .contains(substring));
string_validator!(not_contains, &str, |value: &str, substring: &str| !value
  .contains(substring));
string_validator!(prefix, &str, |value: &str, prefix: &str| value
  .starts_with(prefix));
string_validator!(suffix, &str, |value: &str, suffix: &str| value
  .ends_with(suffix));

well_known_rule!(
  host_and_port,
  "pair of host (hostname or IP address) and port"
);

well_known_rule!(hostname, "hostname");

well_known_rule!(uri, "uri");
well_known_rule!(uri_ref, "URI reference");

well_known_rule!(address, "hostname or ip address");
well_known_rule!(ip, "ip address");
well_known_rule!(ipv4, "ipv4 address");
well_known_rule!(ipv6, "ipv6 address");
well_known_rule!(ip_prefix, "ip prefix");
well_known_rule!(ipv4_prefix, "ipv4 prefix");
well_known_rule!(ipv6_prefix, "ipv6 prefix");
well_known_rule!(ip_with_prefixlen, "ip address with prefix length");
well_known_rule!(ipv4_with_prefixlen, "ipv4 address with prefix length");
well_known_rule!(ipv6_with_prefixlen, "ipv6 address with prefix length");

#[cfg(feature = "regex")]
well_known_rule!(email, "email address");
#[cfg(feature = "regex")]
well_known_rule!(uuid, "uuid");
#[cfg(feature = "regex")]
well_known_rule!(tuuid, "trimmed uuid");

#[cfg(feature = "regex")]
pub fn header_name(
  field_context: &FieldContext,
  value: &str,
  strict: bool,
) -> Result<(), Violation> {
  let check = is_valid_http_header_name(value, strict);

  if check {
    Ok(())
  } else {
    Err(create_violation_with_custom_id(
      "string.well_known_regex.header_name",
      field_context,
      &STRING_WELL_KNOWN_REGEX_VIOLATION,
      "must be a valid HTTP header name",
    ))
  }
}

#[cfg(feature = "regex")]
pub fn header_value(
  field_context: &FieldContext,
  value: &str,
  strict: bool,
) -> Result<(), Violation> {
  let check = is_valid_http_header_value(value, strict);

  if check {
    Ok(())
  } else {
    Err(create_violation_with_custom_id(
      "string.well_known_regex.header_value",
      field_context,
      &STRING_WELL_KNOWN_REGEX_VIOLATION,
      "must be a valid HTTP header value",
    ))
  }
}
