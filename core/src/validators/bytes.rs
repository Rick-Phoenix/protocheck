use bytes::Bytes;
use paste::paste;
use regex::Regex;

use super::well_known_strings::{is_valid_ip, is_valid_ipv4, is_valid_ipv6};
use crate::{
  field_data::FieldContext,
  protovalidate::Violation,
  validators::static_data::{base_violations::create_violation, bytes_violations::*},
};

macro_rules! create_bytes_violation {
  ($check:ident, $field_context:ident, $violation_name:ident, $error_message:expr) => {
    create_violation!(
      bytes,
      $check,
      $field_context,
      $violation_name,
      $error_message
    )
  };
}

macro_rules! well_known_rule {
  (
    $name:ident,
    $definition:literal
  ) => {
    paste! {
      pub fn $name(field_context: &FieldContext, value: &Bytes) -> Result<(), Violation> {
        let string_val = parse_bytes_input(value, field_context)?;
        let check = [<is_valid _ $name>](string_val);

        create_bytes_violation!(check, field_context, $name, concat!("must be a valid ", $definition))
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
    macro_rules! _generate_check {
      (string_arg, $closure:expr, $value:expr, $target:expr, $field_context:expr) => {{
        let string_val = parse_bytes_input($value, $field_context)?;
        $closure($target, &string_val)
      }};

      (bytes_arg, $closure:expr, $value:expr, $target:expr, $field_context:expr) => {
        $closure($target, $value)
      };
    }

    pub fn $name(
      field_context: &FieldContext,
      value: &Bytes,
      target: $target_type,
      error_message: &'static str,
    ) -> Result<(), Violation> {
      let check = _generate_check!($mode, $validation_expression, value, target, field_context);

      create_bytes_violation!(check, field_context, $name, error_message)
    }
  };
}

well_known_rule!(ip, "ip address");
well_known_rule!(ipv4, "ipv4 address");
well_known_rule!(ipv6, "ipv6 address");

bytes_validator!(string_arg, pattern, &Regex, |t: &Regex, s: &str| t
  .is_match(s));

bytes_validator!(bytes_arg, min_len, u64, |t: u64, v: &Bytes| v.len()
  >= t as usize);

bytes_validator!(bytes_arg, max_len, u64, |t: u64, v: &Bytes| v.len()
  <= t as usize);

bytes_validator!(bytes_arg, len, u64, |t: u64, v: &Bytes| v.len()
  == t as usize);

bytes_validator!(
  bytes_arg,
  contains,
  &'static [u8],
  |t: &'static [u8], v: &Bytes| v.windows(t.len()).any(|win| win == t)
);

bytes_validator!(
  bytes_arg,
  suffix,
  &'static [u8],
  |t: &'static [u8], v: &Bytes| v.ends_with(t)
);

bytes_validator!(
  bytes_arg,
  prefix,
  &'static [u8],
  |t: &'static [u8], v: &Bytes| v.starts_with(t)
);
