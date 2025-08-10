use protocheck::types::protovalidate::Violations;

use crate::myapp::v1::WellKnownStrings;

#[test]
fn well_known_strings() {
  let invalid = "they're taking the hobbits \r\n\0 to isengard".to_string();

  let test = WellKnownStrings {
    email: invalid.clone(),
    hostname: invalid.clone(),
    ip: invalid.clone(),
    ipv4: invalid.clone(),
    ipv6: invalid.clone(),
    uri: invalid.clone(),
    uri_ref: invalid.clone(),
    address: invalid.clone(),
    uuid: invalid.clone(),
    tuuid: invalid.clone(),
    ip_with_prefixlen: invalid.clone(),
    ipv4_with_prefixlen: invalid.clone(),
    ipv6_with_prefixlen: invalid.clone(),
    ip_prefix: invalid.clone(),
    ipv4_prefix: invalid.clone(),
    ipv6_prefix: invalid.clone(),
    host_and_port: invalid.clone(),
    header_name_strict: invalid.clone(),
    header_value_strict: invalid.clone(),
    header_name_loose: invalid.clone(),
    header_value_loose: invalid.clone(),
  };

  let Violations { violations } = test.validate().unwrap_err();

  assert_eq!(violations.len(), 21);

  let ipv4 = "192.168.1.1";
  let ipv6 = "2a01:c23:7b6d:a900:1de7:5cbe:d8d2:f4a1";
  let uri = "https://middleeathtracker.com/hobbits?location=isengard";

  let test = WellKnownStrings {
    email: "obiwan@theforce.com".to_string(),
    hostname: "myhost".to_string(),
    ip: ipv4.to_string(),
    ipv4: ipv4.to_string(),
    ipv6: ipv6.to_string(),
    uri: uri.to_string(),
    uri_ref: "./outerspace".to_string(),
    address: "example.com".to_string(),
    uuid: "d3b8f2d5-7e10-4c6e-8a1a-3b9c7d4f6e2c".to_string(),
    tuuid: "d3b8f2d57e104c6e8a1a3b9c7d4f6e2c".to_string(),
    ip_with_prefixlen: format!("{}/16", ipv4),
    ipv4_with_prefixlen: format!("{}/16", ipv4),
    ipv6_with_prefixlen: format!("{}/16", ipv6),
    ip_prefix: "192.168.0.0/16".to_string(),
    ipv4_prefix: "192.168.0.0/16".to_string(),
    ipv6_prefix: "2001:0DB8:ABCD:0012::0/64".to_string(),
    host_and_port: "[2001:0DB8:ABCD:0012::F1]:3000".to_string(),
    header_name_strict: "content-type".to_string(),
    header_value_strict: "application/json".to_string(),
    header_name_loose: "myheader".to_string(),
    header_value_loose: "myheaderval".to_string(),
  };

  assert!(test.validate().is_ok())
}
