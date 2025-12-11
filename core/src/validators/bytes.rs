use ::bytes::Bytes;

use super::{
  well_known_strings::{is_valid_ip, is_valid_ipv4, is_valid_ipv6},
  *,
};
use crate::protovalidate::violations_data::bytes_violations::*;

macro_rules! well_known_rule {
  (
    $name:ident,
    $definition:literal
  ) => {
    paste::paste! {
      pub fn $name(field_context: &FieldContext, value: &Bytes) -> Result<(), Violation> {
        let is_valid = [<is_valid _ $name>](String::from_utf8_lossy(value.as_ref()).as_ref());

        if is_valid {
          Ok(())
        } else {
          Err(create_violation(
            field_context,
            &[< BYTES _ $name:upper _ VIOLATION >],
            concat!("must be a valid ", $definition)
          ))
        }
      }
    }
  };
}

macro_rules! bytes_validator {
  (
    $mode:ident,
    $name:ident,
    $target_type:ty,
    $validation_expression:expr
  ) => {
    pub fn $name(
      field_context: &FieldContext,
      value: &Bytes,
      target: $target_type,
      error_message: &'static str,
    ) -> Result<(), Violation> {
      let is_valid = $validation_expression(target, value);

      if is_valid {
        Ok(())
      } else {
        Err(create_violation(
          field_context,
          paste::paste! {&[< BYTES _ $name:upper _ VIOLATION >]},
          error_message,
        ))
      }
    }
  };
}

well_known_rule!(ip, "ip address");

well_known_rule!(ipv4, "ipv4 address");

well_known_rule!(ipv6, "ipv6 address");

#[cfg(feature = "regex")]
pub fn pattern(
  field_context: &FieldContext,
  value: &Bytes,
  regex: &regex::bytes::Regex,
  error_message: &str,
) -> Result<(), Violation> {
  let is_valid = regex.is_match(value.as_ref());

  if is_valid {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &BYTES_PATTERN_VIOLATION,
      error_message,
    ))
  }
}

bytes_validator!(bytes_arg, min_len, u64, |t: u64, v: &Bytes| v.len() as u64
  >= t);

bytes_validator!(bytes_arg, max_len, u64, |t: u64, v: &Bytes| v.len() as u64
  <= t);

bytes_validator!(bytes_arg, len, u64, |t: u64, v: &Bytes| v.len() as u64 == t);

bytes_validator!(bytes_arg, contains, &[u8], |t: &[u8], v: &Bytes| v
  .windows(t.len())
  .any(|win| win == t));

bytes_validator!(bytes_arg, suffix, &[u8], |t: &[u8], v: &Bytes| v
  .ends_with(t));

bytes_validator!(bytes_arg, prefix, &[u8], |t: &[u8], v: &Bytes| v
  .starts_with(t));
