use std::{
  net::{IpAddr, Ipv6Addr},
  str::FromStr,
  sync::LazyLock,
};

use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};
use iri_string::types::{UriReferenceStr, UriStr};
use regex::Regex;
use uuid::Uuid;

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").expect("Failed to create email regex")
});

pub(crate) fn is_valid_hostname(hostname: &str) -> bool {
  let s = hostname.strip_suffix('.').unwrap_or(hostname);
  if s.len() > 253 {
    return false;
  }

  // Split the hostname into labels.
  let labels: Vec<&str> = s.split('.').collect();

  let last_label = match labels.last() {
    Some(label) => *label,
    None => return false, // Handles empty string case
  };

  // Iterate and validate each label.
  for label in labels {
    // Rule: Each label can be 1 to 63 characters.
    if label.is_empty() || label.len() > 63 {
      return false;
    }

    // Rule: A label can contain hyphens, but must not start or end with one.
    if label.starts_with('-') || label.ends_with('-') {
      return false;
    }

    // Rule: Each label can be alphanumeric characters or hyphens.
    if !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
      return false;
    }
  }

  // Rule: The right-most label must not be digits only.
  if last_label.chars().all(|c| c.is_ascii_digit()) {
    return false;
  }

  true
}

pub(crate) fn is_valid_port(port_str: &str) -> bool {
  // Port must not be empty.
  if port_str.is_empty() {
    return false;
  }
  // Cannot have leading zeros (unless it's just "0").
  if port_str.len() > 1 && port_str.starts_with('0') {
    return false;
  }
  // Try to parse as a u16, which covers the 0-65535 range automatically.
  port_str.parse::<u16>().is_ok()
}

pub(crate) fn is_valid_host_and_port(s: &str) -> bool {
  if s.is_empty() {
    return false;
  }

  if let Some((host_part, port_part)) = s.rsplit_once(':') {
    // Is the host part a bracketed IPv6 address? e.g., `[::1]:8080`
    if let Some(ip_part) = host_part
      .strip_prefix('[')
      .and_then(|s| s.strip_suffix(']'))
    {
      return ip_part.parse::<Ipv6Addr>().is_ok() && is_valid_port(port_part);
    }

    // Otherwise, the host must be a regular hostname or an IP address.
    // `IpAddr` will handle both IPv4 and unbracketed IPv6.
    let is_host_valid = host_part.parse::<IpAddr>().is_ok() || is_valid_hostname(host_part);
    return is_host_valid && is_valid_port(port_part);
  }

  false
}

pub(crate) fn is_valid_ip(s: &str) -> bool {
  s.parse::<IpAddr>().is_ok()
}

pub(crate) fn is_valid_ipv4(s: &str) -> bool {
  s.parse::<IpAddr>().is_ok_and(|ip| ip.is_ipv4())
}

pub(crate) fn is_valid_ipv6(s: &str) -> bool {
  s.parse::<IpAddr>().is_ok_and(|ip| ip.is_ipv6())
}

pub(crate) fn is_valid_uuid(s: &str) -> bool {
  // A standard UUID string with hyphens is always 36 characters long.
  if s.len() != 36 {
    return false;
  }

  Uuid::parse_str(s).is_ok()
}

pub(crate) fn is_valid_tuuid(s: &str) -> bool {
  // A trimmed UUID string is always 32 characters long.
  if s.len() != 32 {
    return false;
  }

  Uuid::parse_str(s).is_ok()
}

pub(crate) fn is_valid_email(s: &str) -> bool {
  EMAIL_REGEX.is_match(s)
}

pub(crate) fn is_valid_uri(s: &str) -> bool {
  <&UriStr>::try_from(s).is_ok()
}

pub(crate) fn is_valid_uri_ref(s: &str) -> bool {
  <&UriReferenceStr>::try_from(s).is_ok()
}

pub(crate) fn is_valid_address(s: &str) -> bool {
  is_valid_hostname(s) || is_valid_ip(s)
}

pub(crate) fn is_valid_ip_prefix(s: &str) -> bool {
  match IpNetwork::from_str(s) {
    Ok(network) => {
      //    Is the given IP address the same as the calculated network address?
      //    The .network() method returns the true prefix.
      //    The .ip() method returns the original IP from the string.
      network.ip() == network.network()
    }
    Err(_) => false,
  }
}

pub(crate) fn is_valid_ipv4_prefix(s: &str) -> bool {
  match Ipv4Network::from_str(s) {
    Ok(network) => network.ip() == network.network(),
    Err(_) => false,
  }
}

pub(crate) fn is_valid_ipv6_prefix(s: &str) -> bool {
  match Ipv6Network::from_str(s) {
    Ok(network) => network.ip() == network.network(),
    Err(_) => false,
  }
}

pub(crate) fn is_valid_ip_with_prefixlen(s: &str) -> bool {
  IpNetwork::from_str(s).is_ok()
}

pub(crate) fn is_valid_ipv4_with_prefixlen(s: &str) -> bool {
  Ipv4Network::from_str(s).is_ok()
}

pub(crate) fn is_valid_ipv6_with_prefixlen(s: &str) -> bool {
  Ipv6Network::from_str(s).is_ok()
}

static HTTP_HEADER_NAME_STRICT_REGEX: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^:?[0-9a-zA-Z!#$%&'*+-.^_|~`]+$").unwrap());

static HTTP_HEADER_NAME_LOOSE_REGEX: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^[^\u0000\u000A\u000D]+$").unwrap());

static HTTP_HEADER_VALUE_STRICT_REGEX: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^[^\x00-\x08\x0A-\x1F\x7F]*$").unwrap());

static HTTP_HEADER_VALUE_LOOSE_REGEX: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^[^\u0000\u000A\u000D]*$").unwrap());

pub(crate) fn is_valid_http_header_name(s: &str, strict: bool) -> bool {
  if s.is_empty() {
    return false;
  }

  let re = if strict {
    &HTTP_HEADER_NAME_STRICT_REGEX
  } else {
    &HTTP_HEADER_NAME_LOOSE_REGEX
  };

  re.is_match(s)
}

pub(crate) fn is_valid_http_header_value(s: &str, strict: bool) -> bool {
  let re = if strict {
    &HTTP_HEADER_VALUE_STRICT_REGEX
  } else {
    &HTTP_HEADER_VALUE_LOOSE_REGEX
  };

  re.is_match(s)
}

#[cfg(test)]
mod test {
  use crate::validators::well_known_strings::{
    is_valid_http_header_name, is_valid_http_header_value,
  };

  #[test]
  fn headers() {
    assert!(is_valid_http_header_name("content-type", true));
    assert!(is_valid_http_header_name(":authority", true));

    assert!(!is_valid_http_header_name("content type", true));
    assert!(!is_valid_http_header_name("X-My@Header", true));
    assert!(!is_valid_http_header_name("X-Héader", true));
    assert!(!is_valid_http_header_name("", true));

    assert!(is_valid_http_header_name("X-My@Header", false));
    assert!(is_valid_http_header_name("X-Héader", false));

    assert!(!is_valid_http_header_name("Header\u{0000}WithNul", false));
    assert!(!is_valid_http_header_name("Header\nWithNewline", false));
    assert!(!is_valid_http_header_name("header\rwithcr", false));
    assert!(!is_valid_http_header_name("", false));

    assert!(is_valid_http_header_value(
      "application/json; charset=uft-8",
      true
    ));

    assert!(!is_valid_http_header_value(
      "value\u{0000}with\u{0000}nul",
      true
    ));
    assert!(!is_valid_http_header_value(
      "value\u{0007}with\u{0007}bell",
      true
    ));
    assert!(!is_valid_http_header_value(
      "value\u{000B}with\u{000B}vt",
      true
    ));
    assert!(!is_valid_http_header_value(
      "value\u{007F}with\u{007F}del",
      true
    ));

    assert!(!is_valid_http_header_value(
      "value\u{0000}with\u{0000}nul",
      false
    ));
    assert!(!is_valid_http_header_value("value\nwith\nnewline", false));
    assert!(!is_valid_http_header_value("value\rwith\rcr", false));
  }
}
