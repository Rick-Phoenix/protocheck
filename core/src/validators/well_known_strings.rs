use std::net::{IpAddr, Ipv6Addr};

pub(crate) fn is_valid_uri(s: &str) -> bool {
  fluent_uri::Uri::parse(s).is_ok()
}

pub(crate) fn is_valid_uri_ref(s: &str) -> bool {
  fluent_uri::UriRef::parse(s).is_ok()
}

#[cfg(feature = "regex")]
mod regex {
  use std::sync::LazyLock;

  use regex::Regex;

  static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").expect("Failed to create email regex")
  });

  pub(crate) fn is_valid_email(s: &str) -> bool {
    EMAIL_REGEX.is_match(s)
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
    if s.is_empty() {
      return false;
    }

    let re = if strict {
      &HTTP_HEADER_VALUE_STRICT_REGEX
    } else {
      &HTTP_HEADER_VALUE_LOOSE_REGEX
    };

    re.is_match(s)
  }

  static UUID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?i)[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap()
  });

  static TUUID_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?i)[0-9a-f]{32}$").unwrap());

  pub(crate) fn is_valid_uuid(s: &str) -> bool {
    if s.is_empty() {
      return false;
    }

    UUID_REGEX.is_match(s)
  }

  pub(crate) fn is_valid_tuuid(s: &str) -> bool {
    if s.is_empty() {
      return false;
    }

    TUUID_REGEX.is_match(s)
  }
}

use std::str::FromStr;

use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};
#[cfg(feature = "regex")]
pub(crate) use regex::*;

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

pub(crate) fn is_valid_ip(s: &str) -> bool {
  s.parse::<IpAddr>().is_ok()
}

pub(crate) fn is_valid_ipv4(s: &str) -> bool {
  s.parse::<IpAddr>().is_ok_and(|ip| ip.is_ipv4())
}

pub(crate) fn is_valid_ipv6(s: &str) -> bool {
  s.parse::<IpAddr>().is_ok_and(|ip| ip.is_ipv6())
}

pub(crate) fn is_valid_address(s: &str) -> bool {
  is_valid_hostname(s) || is_valid_ip(s)
}

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

#[cfg(test)]
mod test {
  use crate::validators::well_known_strings::{
    is_valid_address, is_valid_host_and_port, is_valid_hostname, is_valid_ip, is_valid_ipv4,
    is_valid_ipv6,
  };

  #[test]
  fn uris() {
    use crate::validators::well_known_strings::is_valid_uri;

    assert!(is_valid_uri(
      "https://middleeathtracker.com/hobbits?location=isengard"
    ));

    assert!(!is_valid_uri(
      "https://middleeathtracker.com/hobbits?location isengard"
    ));
  }

  #[test]
  fn name() {
    use crate::validators::well_known_strings::{
      is_valid_ip_prefix, is_valid_ip_with_prefixlen, is_valid_ipv4_prefix,
      is_valid_ipv4_with_prefixlen, is_valid_ipv6_prefix, is_valid_ipv6_with_prefixlen,
    };

    let ipv4_prefix = "192.168.0.0/16";
    let ipv4_with_prefixlen = "192.168.1.1/16";
    let ipv6_prefix = "2a01:c00::/24";
    let ipv6_with_prefixlen = "2a01:c23:7b6d:a900:1de7:5cbe:d8d2:f4a1/24";

    assert!(is_valid_ip_with_prefixlen(ipv4_with_prefixlen));
    assert!(is_valid_ip_with_prefixlen(ipv6_with_prefixlen));
    assert!(is_valid_ipv4_with_prefixlen(ipv4_with_prefixlen));
    assert!(!is_valid_ipv4_with_prefixlen(ipv6_with_prefixlen));
    assert!(is_valid_ipv6_with_prefixlen(ipv6_with_prefixlen));
    assert!(!is_valid_ipv6_with_prefixlen(ipv4_with_prefixlen));

    assert!(is_valid_ip_prefix(ipv4_prefix));
    assert!(is_valid_ip_prefix(ipv6_prefix));
    assert!(is_valid_ipv4_prefix(ipv4_prefix));
    assert!(!is_valid_ipv4_prefix(ipv6_prefix));
    assert!(!is_valid_ipv4_prefix(ipv4_with_prefixlen));
    assert!(is_valid_ipv6_prefix(ipv6_prefix));
    assert!(!is_valid_ipv6_prefix(ipv4_prefix));
    assert!(!is_valid_ipv6_prefix(ipv6_with_prefixlen));
  }

  #[test]
  fn network_identifiers() {
    let ipv4 = "192.168.1.1";
    let ipv6 = "2a01:c23:7b6d:a900:1de7:5cbe:d8d2:f4a1";

    assert!(is_valid_ip(ipv4));
    assert!(is_valid_ip(ipv6));
    assert!(is_valid_ipv4(ipv4));
    assert!(!is_valid_ipv4(ipv6));
    assert!(is_valid_ipv6(ipv6));
    assert!(!is_valid_ipv6(ipv4));

    assert!(is_valid_address("obiwan.force.com"));
    assert!(is_valid_address(ipv4));
    assert!(is_valid_address(ipv6));

    assert!(is_valid_host_and_port("obiwan.force:8080"));
    assert!(is_valid_host_and_port("192.168.1.120:3000"));
    assert!(is_valid_host_and_port("[2001:0DB8:ABCD:0012::F1]:3000"));

    assert!(!is_valid_host_and_port("obiwan.force"));
    assert!(!is_valid_host_and_port("192.168.1.120"));
    assert!(!is_valid_host_and_port("2001:0DB8:ABCD:0012::F1"));

    assert!(is_valid_hostname("obiwan.force.com"));
    assert!(!is_valid_hostname("-anakin.darkforce.com"));
    assert!(!is_valid_hostname("anakin.darkforce.com-"));
    assert!(!is_valid_hostname("anakin.darkforce.0"));
  }

  #[cfg(feature = "regex")]
  #[test]
  fn identifiers() {
    use crate::validators::well_known_strings::{is_valid_email, is_valid_tuuid, is_valid_uuid};

    assert!(is_valid_email("obiwan@force.com"));
    assert!(!is_valid_email("anakin@dark@force.com"));

    assert!(is_valid_uuid("d3b8f2d5-7e10-4c6e-8a1a-3b9c7d4f6e2c"));
    assert!(!is_valid_uuid("d3b8f2d57e104c6e8a1a3b9c7d4f6e2c"));

    assert!(is_valid_tuuid("d3b8f2d57e104c6e8a1a3b9c7d4f6e2c"));
    assert!(!is_valid_tuuid("d3b8f2d5-7e10-4c6e-8a1a-3b9c7d4f6e2c"))
  }

  #[cfg(feature = "regex")]
  #[test]
  fn headers() {
    use crate::validators::well_known_strings::{
      is_valid_http_header_name, is_valid_http_header_value,
    };

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
