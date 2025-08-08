use proto_types::protovalidate::Ignore;
use regex::Regex;

use crate::{
  field_data::FieldContext,
  protovalidate::Violation,
  validators::static_data::{
    base_violations::create_violation,
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
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_http_header_name(value, strict);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_WELL_KNOWN_REGEX_VIOLATION,
      "string.well_known_regex.header_name",
      "must be a valid HTTP header name",
    ))
  }
}

pub fn header_value(
  field_context: &FieldContext,
  value: &str,
  strict: bool,
) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_http_header_value(value, strict);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_WELL_KNOWN_REGEX_VIOLATION,
      "string.well_known_regex.header_value",
      "must be a valid HTTP header value",
    ))
  }
}

pub fn host_and_port(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_host_and_port(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_HOST_AND_PORT_VIOLATION,
      "string.host_and_port",
      "must be a valid pair of host (hostname or IP address) and port",
    ))
  }
}

pub fn ip_prefix(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_ip_prefix(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_IP_PREFIX_VIOLATION,
      "string.ip_prefix",
      "must be a valid ip prefix",
    ))
  }
}

pub fn ipv4_prefix(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_ipv4_prefix(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_IPV4_PREFIX_VIOLATION,
      "string.ipv4_prefix",
      "must be a valid ipv4 prefix",
    ))
  }
}

pub fn ipv6_prefix(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_ipv6_prefix(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_IPV6_PREFIX_VIOLATION,
      "string.ipv6_prefix",
      "must be a valid ipv6 prefix",
    ))
  }
}

pub fn ip_with_prefix_len(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_ip_with_prefixlen(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_IP_WITH_PREFIX_LEN_VIOLATION,
      "string.ip_with_prefixlen",
      "must be a valid ip address with prefix length",
    ))
  }
}

pub fn ipv6_with_prefix_len(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_ipv6_with_prefixlen(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_IPV6_WITH_PREFIX_LEN_VIOLATION,
      "string.ipv6_with_prefixlen",
      "must be a valid ipv6 address with prefix length",
    ))
  }
}

pub fn ipv4_with_prefix_len(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_ipv4_with_prefixlen(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_IPV4_WITH_PREFIX_LEN_VIOLATION,
      "string.ipv4_with_prefixlen",
      "must be a valid ipv4 address with prefix length",
    ))
  }
}

pub fn tuuid(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_trimmed_uuid(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_TUUID_VIOLATION,
      "string.tuuid",
      "must be a valid trimmed uuid",
    ))
  }
}

pub fn uuid(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_uuid(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_UUID_VIOLATION,
      "string.uuid",
      "must be a valid uuid",
    ))
  }
}

pub fn address(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_address(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_ADDRESS_VIOLATION,
      "string.address",
      "must be a valid hostname or ip address",
    ))
  }
}

pub fn uri_ref(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_uri_ref(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_URI_REF_VIOLATION,
      "string.uri_ref",
      "must be a valid URI reference",
    ))
  }
}

pub fn uri(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_uri(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_URI_VIOLATION,
      "string.uri",
      "must be a valid URI",
    ))
  }
}

pub fn hostname(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_hostname(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_HOSTNAME_VIOLATION,
      "string.hostname",
      "must be a valid hostname",
    ))
  }
}

pub fn ip(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_ip(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_IP_VIOLATION,
      "string.ip",
      "must be a valid ip address",
    ))
  }
}

pub fn ipv4(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_ipv4(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_IPV4_VIOLATION,
      "string.ipv4",
      "must be a valid ipv4 address",
    ))
  }
}

pub fn ipv6(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_ipv6(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_IPV6_VIOLATION,
      "string.ipv6",
      "must be a valid ipv6 address",
    ))
  }
}

pub fn email(field_context: &FieldContext, value: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = is_valid_email(value);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &STRING_EMAIL_VIOLATION,
      "string.email",
      "must be a valid email address",
    ))
  }
}

pub fn pattern(
  field_context: &FieldContext,
  value: &str,
  pattern: &Regex,
) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = pattern.is_match(value);

  if check {
    Ok(())
  } else {
    let error_message = format!("must match the following regex: `{}`", pattern);
    Err(create_violation(
      field_context,
      &STRING_PATTERN_VIOLATION,
      "string.pattern",
      &error_message,
    ))
  }
}

pub fn contains(field_context: &FieldContext, value: &str, pattern: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = value.contains(pattern);

  if check {
    Ok(())
  } else {
    let error_message = format!("must contain the '{}' substring", pattern);
    Err(create_violation(
      field_context,
      &STRING_CONTAINS_VIOLATION,
      "string.contains",
      &error_message,
    ))
  }
}

pub fn not_contains(
  field_context: &FieldContext,
  value: &str,
  pattern: &str,
) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = !value.contains(pattern);

  if check {
    Ok(())
  } else {
    let error_message = format!("must not contain the '{}' substring", pattern);
    Err(create_violation(
      field_context,
      &STRING_NOT_CONTAINS_VIOLATION,
      "string.not_contains",
      &error_message,
    ))
  }
}

pub fn prefix(field_context: &FieldContext, value: &str, prefix: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = value.starts_with(prefix);

  if check {
    Ok(())
  } else {
    let error_message = format!("must start with '{}'", prefix,);
    Err(create_violation(
      field_context,
      &STRING_PREFIX_VIOLATION,
      "string.prefix",
      &error_message,
    ))
  }
}

pub fn suffix(field_context: &FieldContext, value: &str, suffix: &str) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = value.ends_with(suffix);

  if check {
    Ok(())
  } else {
    let error_message = format!("must end with '{}'", suffix,);
    Err(create_violation(
      field_context,
      &STRING_SUFFIX_VIOLATION,
      "string.suffix",
      &error_message,
    ))
  }
}

pub fn max_len(field_context: &FieldContext, value: &str, max_len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = value.chars().count() <= max_len as usize;

  if check {
    Ok(())
  } else {
    let plural_suffix = if max_len > 1 { "s" } else { "" };
    let error_message = format!(
      "cannot be longer than {} character{}",
      max_len, plural_suffix
    );
    Err(create_violation(
      field_context,
      &STRING_MAX_LEN_VIOLATION,
      "string.max_len",
      &error_message,
    ))
  }
}

pub fn min_len(field_context: &FieldContext, value: &str, min_len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = value.chars().count() >= min_len as usize;

  if check {
    Ok(())
  } else {
    let plural_suffix = if min_len > 1 { "s" } else { "" };
    let error_message = format!(
      "cannot be shorter than {} character{}",
      min_len, plural_suffix
    );
    Err(create_violation(
      field_context,
      &STRING_MIN_LEN_VIOLATION,
      "string.min_len",
      &error_message,
    ))
  }
}

pub fn len(field_context: &FieldContext, value: &str, len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = value.chars().count() == len as usize;

  if check {
    Ok(())
  } else {
    let plural_suffix = if len > 1 { "s" } else { "" };
    let error_message = format!("must be exactly {} character{} long", len, plural_suffix);
    Err(create_violation(
      field_context,
      &STRING_LEN_VIOLATION,
      "string.len",
      &error_message,
    ))
  }
}

pub fn len_bytes(field_context: &FieldContext, value: &str, len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = value.len() == len as usize;

  if check {
    Ok(())
  } else {
    let plural_suffix = if len > 1 { "s" } else { "" };
    let error_message = format!("must be exactly {} byte{} long", len, plural_suffix);
    Err(create_violation(
      field_context,
      &STRING_LEN_BYTES_VIOLATION,
      "string.len_bytes",
      &error_message,
    ))
  }
}

pub fn min_bytes(
  field_context: &FieldContext,
  value: &str,
  min_bytes: u64,
) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = value.len() >= min_bytes as usize;

  if check {
    Ok(())
  } else {
    let plural_suffix = if min_bytes > 1 { "s" } else { "" };
    let error_message = format!("cannot be shorter than {} byte{}", min_bytes, plural_suffix);
    Err(create_violation(
      field_context,
      &STRING_MIN_BYTES_VIOLATION,
      "string.min_bytes",
      &error_message,
    ))
  }
}

pub fn max_bytes(field_context: &FieldContext, value: &str, max_len: u64) -> Result<(), Violation> {
  if let Ignore::IfZeroValue = field_context.ignore
    && value.is_empty() {
      return Ok(());
    }

  let check = value.len() <= max_len as usize;

  if check {
    Ok(())
  } else {
    let plural_suffix = if max_len > 1 { "s" } else { "" };
    let error_message = format!("cannot be longer than {} byte{}", max_len, plural_suffix);
    Err(create_violation(
      field_context,
      &STRING_MAX_BYTES_VIOLATION,
      "string.max_bytes",
      &error_message,
    ))
  }
}
