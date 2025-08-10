use bytes::Bytes;
use protocheck::types::protovalidate::Violations;

use crate::myapp::v1::BytesTests;

#[test]
fn bytes_tests() {
  let example1 = b"they're taking the hobbits to isengard";

  let test = BytesTests {
    contains_field: Bytes::from_static(example1),
    prefix_field: Bytes::from_static(example1),
    pattern_field: Bytes::from_static(example1),
    suffix_field: Bytes::from_static(example1),
    ip: Bytes::from_static(example1),
    ipv4: Bytes::from_static(example1),
    ipv6: Bytes::from_static(example1),
  };

  let Violations { violations } = test.validate().unwrap_err();

  assert_eq!(violations.len(), 7);

  let correct = b"abc";
  let valid_ipv4 = b"192.168.1.0";
  let valid_ipv6 = b"2a01:c23:7b6d:a900:1de7:5cbe:d8d2:f4a1";

  let test = BytesTests {
    contains_field: Bytes::from_static(correct),
    prefix_field: Bytes::from_static(correct),
    pattern_field: Bytes::from_static(correct),
    suffix_field: Bytes::from_static(correct),
    ip: Bytes::from_static(valid_ipv4),
    ipv4: Bytes::from_static(valid_ipv4),
    ipv6: Bytes::from_static(valid_ipv6),
  };

  assert!(test.validate().is_ok())
}
