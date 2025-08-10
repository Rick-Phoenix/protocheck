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

  let _: Vec<_> = violations
    .iter()
    .map(|v| println!("{}", v.rule_id()))
    .collect();
  assert_eq!(violations.len(), 21);
}
